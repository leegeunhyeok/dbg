#[cfg(target_arch = "wasm32")]
use swc_common::SourceMapper;
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

use crate::types::{DbgArg, Pos};

const NL: &str = "\n";
const SEMICOL: &str = ";";
const DBG_EXP_MEMBER: &str = "_";
const DBG_RUNTIME: &str = "unplugin-dbg/runtime";

pub struct DbgTransformer {
    #[allow(dead_code)]
    // Referenced in `get_pos`
    sm_proxy: PluginSourceMapProxy,
    unresolved_ctxt: SyntaxContext,
    dbg_ident: Ident,
    has_dbg_call: bool,
}

impl DbgTransformer {
    pub fn new(sm_proxy: PluginSourceMapProxy, unresolved_ctxt: SyntaxContext) -> Self {
        Self {
            sm_proxy,
            unresolved_ctxt,
            dbg_ident: private_ident!("__dbg"),
            has_dbg_call: false,
        }
    }

    fn is_dbg_call(&self, call_expr: &CallExpr) -> bool {
        match &call_expr.callee {
            Callee::Expr(expr) => {
                if let Some(ident) = expr.as_ident() {
                    ident.ctxt == self.unresolved_ctxt
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn expr_to_str(&self, expr: &Expr) -> String {
        let mut buf = vec![];
        let cm: Lrc<SourceMap> = Default::default();
        let wr = Box::new(JsWriter::new(cm.clone(), NL, &mut buf, None));

        let mut emitter = Emitter {
            cfg: Default::default(),
            comments: None,
            wr,
            cm,
        };

        emitter
            .emit_script(&Script {
                span: Default::default(),
                shebang: None,
                body: vec![Stmt::Expr(ExprStmt {
                    expr: Box::new(expr.clone()),
                    ..Default::default()
                })],
            })
            .unwrap();

        String::from_utf8(buf)
            .unwrap()
            .trim_end_matches(NL)
            .trim_end_matches(SEMICOL)
            .to_string()
    }

    #[allow(unused_variables)]
    fn get_pos(&self, span: Span) -> Option<Pos> {
        // `lookup_char_pos` is only available in `wasm32` target.
        #[cfg(target_arch = "wasm32")]
        {
            let loc = self.sm_proxy.lookup_char_pos(span.lo());
            return Some(Pos(loc.line, loc.col.0));
        }
        #[allow(unreachable_code)]
        None
    }

    fn to_dbg_arg(&self, arg: ExprOrSpread) -> DbgArg {
        let expr_str = self.expr_to_str(&*arg.expr);

        DbgArg::new(expr_str, *arg.expr)
    }
}

impl VisitMut for DbgTransformer {
    noop_visit_mut_type!();

    fn visit_mut_script(&mut self, script: &mut Script) {
        script.visit_mut_children_with(self);

        if self.has_dbg_call {
            let require_call = quote_ident!("require").as_call(
                DUMMY_SP,
                vec![Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: DBG_RUNTIME.into(),
                    raw: None,
                }))
                .as_arg()],
            );

            let require_var_decl = require_call.into_var_decl(
                VarDeclKind::Const,
                Pat::Object(ObjectPat {
                    props: vec![ObjectPatProp::KeyValue(KeyValuePatProp {
                        key: PropName::Ident(IdentName {
                            span: DUMMY_SP,
                            sym: DBG_EXP_MEMBER.into(),
                        }),
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

        if self.has_dbg_call {
            module.body.insert(
                0,
                ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                    phase: ImportPhase::Evaluation,
                    specifiers: vec![ImportSpecifier::Named(ImportNamedSpecifier {
                        span: DUMMY_SP,
                        local: self.dbg_ident.clone(),
                        imported: Some(ModuleExportName::Ident(
                            quote_ident!(DBG_EXP_MEMBER).into(),
                        )),
                        is_type_only: false,
                    })],
                    src: Box::new(Str {
                        span: DUMMY_SP,
                        value: DBG_RUNTIME.into(),
                        raw: None,
                    }),
                    type_only: false,
                    with: None,
                    span: DUMMY_SP,
                })),
            );
        }
    }

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        if let Expr::Call(call_expr) = expr {
            if self.is_dbg_call(call_expr) {
                let pos = self.get_pos(call_expr.span);
                let mut args = Vec::with_capacity(call_expr.args.len() + 1);

                args.push(
                    pos.map_or(Expr::Lit(Lit::Null(Null { span: DUMMY_SP })), |pos| {
                        pos.into_obj_lit().into()
                    })
                    .into(),
                );
                args.extend(
                    call_expr
                        .args
                        .drain(..)
                        .map(|arg| self.to_dbg_arg(arg).into_arg()),
                );

                *expr = self
                    .dbg_ident
                    .clone()
                    .make_member(IdentName {
                        span: Default::default(),
                        sym: "call".into(),
                    })
                    .as_call(DUMMY_SP, args);

                self.has_dbg_call = true;
            }
        }
    }
}
