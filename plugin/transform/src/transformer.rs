#[cfg(target_arch = "wasm32")]
use swc_common::{FileName, SourceMapper};
#[cfg(not(target_arch = "wasm32"))]
use swc_core::ecma::ast::{Lit, Null};
use swc_core::{
    common::{sync::Lrc, SourceMap, Span, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::{
            CallExpr, Callee, Expr, ExprOrSpread, Ident, ImportDecl, ImportNamedSpecifier,
            ImportPhase, ImportSpecifier, KeyValuePatProp, Module, ModuleDecl, ModuleExportName,
            ModuleItem, ObjectPat, ObjectPatProp, Pat, PropName, Script, VarDeclKind,
        },
        codegen::{text_writer::JsWriter, Config, Emitter},
        utils::{private_ident, quote_ident, ExprFactory},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
    plugin::proxies::PluginSourceMapProxy,
};

use crate::types::DbgArg;
#[cfg(target_arch = "wasm32")]
use crate::types::Loc;

const DBG_RUNTIME_SRC: &str = "unplugin-dbg/runtime";
const DBG_RUNTIME_SHIM_SRC: &str = "unplugin-dbg/runtime-shim";
const DBG_EXP_MEMBER: &str = "_";
const NL: &str = "\n";

pub(crate) struct DbgTransformer {
    cm: PluginSourceMapProxy,
    unresolved_ctxt: SyntaxContext,
    enabled: bool,
    dbg_ident: Ident,
    has_dbg_call: bool,
    runtime_src: &'static str,
}

impl DbgTransformer {
    pub(crate) fn new(
        cm: PluginSourceMapProxy,
        unresolved_ctxt: SyntaxContext,
        enabled: bool,
    ) -> Self {
        Self {
            cm,
            unresolved_ctxt,
            enabled,
            dbg_ident: private_ident!("__dbg"),
            has_dbg_call: false,
            runtime_src: if enabled {
                DBG_RUNTIME_SRC
            } else {
                DBG_RUNTIME_SHIM_SRC
            },
        }
    }

    /// Check if the given `CallExpr` is a `dbg` function call.
    ///
    /// ```js
    /// dbg(); // true
    ///
    /// function (dbg) {
    ///   dbg(); // false
    /// }
    ///
    /// var dbg;
    /// dbg(); // false
    /// ```
    fn is_dbg_call(&self, call_expr: &CallExpr) -> bool {
        match &call_expr.callee {
            // Only handle `dbg` when the identifier is unresolved.
            Callee::Expr(callee_expr) => callee_expr
                .as_ident()
                .is_some_and(|ident| ident.sym == "dbg" && ident.ctxt == self.unresolved_ctxt),
            _ => false,
        }
    }
}

/// Converts an expression to its JavaScript source representation.
fn expression_to_string(expr: &Expr) -> String {
    let cm: Lrc<SourceMap> = Lrc::default();
    let mut buffer = Vec::new();
    let mut writer = JsWriter::new(cm.clone(), NL, &mut buffer, None);
    writer.set_indent_str("  ");

    let mut emitter = Emitter {
        cfg: Config::default(),
        comments: None,
        wr: Box::new(writer),
        cm,
    };

    emitter
        .emit_script(&Script {
            span: Span::default(),
            shebang: None,
            body: vec![expr.clone().into_stmt()],
        })
        .expect("emitting JavaScript to an in-memory buffer should not fail");

    String::from_utf8(buffer)
        .expect("the SWC code generator should emit valid UTF-8")
        .trim_end_matches(NL)
        .trim_end_matches(';')
        .to_owned()
}

#[cfg(target_arch = "wasm32")]
fn location_expression(cm: &PluginSourceMapProxy, span: Span) -> Expr {
    let loc = cm.lookup_char_pos(span.lo());
    let file = match loc.file.name.as_ref() {
        FileName::Real(path) => path.to_string_lossy().into_owned(),
        _ => "<anonymous>".to_owned(),
    };

    Loc::new(file, loc.line, loc.col.0 + 1)
        .into_obj_lit()
        .into()
}

#[cfg(not(target_arch = "wasm32"))]
const fn location_expression(_cm: &PluginSourceMapProxy, _span: Span) -> Expr {
    Expr::Lit(Lit::Null(Null { span: DUMMY_SP }))
}

/// Converts an expression argument to the runtime representation.
fn to_dbg_arg(arg: ExprOrSpread) -> DbgArg {
    let expr = expression_to_string(&arg.expr);

    DbgArg::new(expr, *arg.expr)
}

impl VisitMut for DbgTransformer {
    noop_visit_mut_type!();

    fn visit_mut_script(&mut self, script: &mut Script) {
        script.visit_mut_children_with(self);

        // Insert require call expression at the top of the script.
        //
        // ```js
        // const { _: __dbg } = require('unplugin-dbg/runtime');
        // ```
        if self.has_dbg_call {
            let require_call =
                quote_ident!("require").as_call(DUMMY_SP, vec![self.runtime_src.as_arg()]);

            let require_var_decl = require_call.into_var_decl(
                VarDeclKind::Const,
                Pat::Object(ObjectPat {
                    props: vec![ObjectPatProp::KeyValue(KeyValuePatProp {
                        key: PropName::Ident(DBG_EXP_MEMBER.into()),
                        value: Box::new(self.dbg_ident.clone().into()),
                    })],
                    optional: false,
                    type_ann: None,
                    span: DUMMY_SP,
                }),
            );

            script.body.insert(0, require_var_decl.into());
        }
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        module.visit_mut_children_with(self);

        // Insert import declaration at the top of the module.
        //
        // ```js
        // import { _ as __dbg } from 'unplugin-dbg/runtime';
        // ```
        if self.has_dbg_call {
            module.body.insert(
                0,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    phase: ImportPhase::Evaluation,
                    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                        local: self.dbg_ident.clone(),
                        imported: Some(ModuleExportName::Ident(
                            quote_ident!(DBG_EXP_MEMBER).into(),
                        )),
                        is_type_only: false,
                        span: DUMMY_SP,
                    })],
                    src: Box::new(self.runtime_src.into()),
                    type_only: false,
                    with: None,
                    span: DUMMY_SP,
                })),
            );
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        expr.visit_mut_children_with(self);

        if let Expr::Call(call_expr) = expr {
            if self.is_dbg_call(call_expr) {
                // Set flag to `true` to insert import/require at the top of the module/script.
                self.has_dbg_call = true;

                if self.enabled {
                    let mut args = Vec::with_capacity(call_expr.args.len() + 1);

                    // Context object (Location)
                    args.push(location_expression(&self.cm, call_expr.span).into());

                    // Converted arguments
                    args.extend(
                        std::mem::take(&mut call_expr.args)
                            .into_iter()
                            .map(|arg| to_dbg_arg(arg).into_arg()),
                    );

                    // Call `__dbg.call` with the converted arguments
                    *expr = self
                        .dbg_ident
                        .clone()
                        .make_member("call".into())
                        .as_call(DUMMY_SP, args);
                } else {
                    // If disabled, keep the original arguments and call the runtime shim.
                    *expr = self
                        .dbg_ident
                        .clone()
                        .as_call(DUMMY_SP, std::mem::take(&mut call_expr.args));
                }
            }
        }
    }
}
