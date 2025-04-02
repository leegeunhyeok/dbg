#[cfg(target_arch = "wasm32")]
use swc_common::{FileName, SourceMapper};
use swc_core::{
    common::{sync::Lrc, SourceMap, Span, SyntaxContext, DUMMY_SP},
    ecma::{
        ast::*,
        codegen::{text_writer::JsWriter, Emitter},
        utils::{private_ident, quote_ident, ExprFactory},
        visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
    },
    plugin::proxies::PluginSourceMapProxy,
};

use crate::types::{DbgArg, Loc};

const DBG_RUNTIME_SRC: &str = "unplugin-dbg/runtime";
const DBG_RUNTIME_SHIM_SRC: &str = "unplugin-dbg/runtime-shim";
const DBG_EXP_MEMBER: &str = "_";
const NL: &str = "\n";

pub struct DbgTransformer {
    #[allow(dead_code)]
    // Referenced in `get_loc`
    cm: PluginSourceMapProxy,
    unresolved_ctxt: SyntaxContext,
    enabled: bool,
    dbg_ident: Ident,
    has_dbg_call: bool,
    runtime_src: &'static str,
}

impl DbgTransformer {
    pub fn new(cm: PluginSourceMapProxy, unresolved_ctxt: SyntaxContext, enabled: bool) -> Self {
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
            Callee::Expr(callee_expr) => {
                // Only handle `dbg` function when it's unresolved.
                callee_expr.is_ident_ref_to("dbg")
                    && callee_expr.as_ident().unwrap().ctxt == self.unresolved_ctxt
            }
            _ => false,
        }
    }

    /// Convert `Expr` to JavaScript string.
    ///
    /// ```js
    /// a + b + 10 + "hello"
    /// // Into
    /// "a + b + 10 + 'hello'"
    /// ```
    fn expr_to_str(&self, expr: &Expr) -> String {
        let cm: Lrc<SourceMap> = Default::default();
        let mut buf = vec![];
        let mut wr = JsWriter::new(cm.clone(), NL, &mut buf, None);
        wr.set_indent_str("  " /* 2 spaces */);

        let mut emitter = Emitter {
            cfg: Default::default(),
            comments: None,
            wr: Box::new(wr),
            cm,
        };

        emitter
            .emit_script(&Script {
                span: Default::default(),
                shebang: None,
                body: vec![expr.clone().into_stmt()],
            })
            .unwrap();

        // Convert to string and strip newline and semicolon at the end
        String::from_utf8(buf)
            .unwrap()
            .trim_end_matches(NL)
            .trim_end_matches(";")
            .to_string()
    }

    /// Get location of the given span
    #[allow(unused_variables)]
    fn get_loc(&self, span: Span) -> Option<Loc> {
        // `lookup_char_pos` is only available in `wasm32` target.
        #[cfg(target_arch = "wasm32")]
        {
            let loc = self.cm.lookup_char_pos(span.lo());
            let file = match loc.file.name.as_ref() {
                FileName::Real(path) => path.to_string_lossy().to_string(),
                _ => String::from("<anonymous>"),
            };

            return Some(Loc(
                file,
                loc.line,
                loc.col.0 + 1, /* Add 1 for zero-based col index */
            ));
        }
        #[allow(unreachable_code)]
        None
    }

    /// Convert `ExprOrSpread` to `DbgArg`
    fn to_dbg_arg(&self, arg: ExprOrSpread) -> DbgArg {
        let expr_str = self.expr_to_str(&*arg.expr);

        DbgArg::new(expr_str, *arg.expr)
    }
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

                if self.enabled == false {
                    // If plugin is disabled, replace the call with shim of `__dbg` with keeping original arguments
                    *expr = self
                        .dbg_ident
                        .clone()
                        .as_call(DUMMY_SP, call_expr.args.drain(..).collect());
                } else {
                    let loc = self.get_loc(call_expr.span);
                    let mut args = Vec::with_capacity(call_expr.args.len() + 1);

                    // Context object (Location)
                    args.push(
                        loc.map_or(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })), |loc| {
                            loc.into_obj_lit().into()
                        })
                        .into(),
                    );

                    // Converted arguments
                    args.extend(
                        call_expr
                            .args
                            .drain(..)
                            .map(|arg| self.to_dbg_arg(arg).into_arg()),
                    );

                    // Call `__dbg.call` with the converted arguments
                    *expr = self
                        .dbg_ident
                        .clone()
                        .make_member("call".into())
                        .as_call(DUMMY_SP, args);
                }
            } else {
                // Visit children of non `dbg` call expressions to find nested `dbg` calls.
                //
                // ```js
                // foo(() => dbg('nested'));
                //
                // // 1. `foo` is non `dbg` call
                // // 2. traverse children nodes of `foo`
                // // 3. find `dbg` call -> handle the AST node with self.visit_mut_expr again
                // ```
                call_expr.visit_mut_children_with(self);
            }
        }
    }
}
