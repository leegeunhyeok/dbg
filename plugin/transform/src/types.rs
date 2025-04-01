use swc_common::DUMMY_SP;
use swc_core::ecma::ast::*;

pub struct DbgArg {
    expr: String,
    orig: Expr,
}

impl DbgArg {
    pub fn new(expr: String, orig: Expr) -> Self {
        Self { expr, orig }
    }

    pub fn into_arg(self) -> ExprOrSpread {
        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Object(ObjectLit {
                props: vec![
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(IdentName {
                            sym: "expr".into(),
                            ..Default::default()
                        }),
                        value: Box::new(Expr::Lit(Lit::Str(Str::from(self.expr)))),
                    }))),
                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                        key: PropName::Ident(IdentName {
                            sym: "value".into(),
                            ..Default::default()
                        }),
                        value: Box::new(self.orig),
                    }))),
                ],
                ..Default::default()
            })),
        }
    }
}

pub struct Loc(pub String, pub usize, pub usize);

impl Loc {
    fn obj_prop(&self, key: &str, value: Expr) -> Prop {
        Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(IdentName {
                span: DUMMY_SP,
                sym: key.into(),
            }),
            value: Box::new(value),
        })
    }

    pub fn into_obj_lit(self) -> ObjectLit {
        let Self(ref file, line, col) = self;

        ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(
                    self.obj_prop("file", Expr::Lit(Lit::Str(file.to_owned().into()))),
                )),
                PropOrSpread::Prop(Box::new(self.obj_prop(
                    "line",
                    Expr::Lit(Lit::Num(Number {
                        value: line as f64,
                        raw: None,
                        span: DUMMY_SP,
                    })),
                ))),
                PropOrSpread::Prop(Box::new(self.obj_prop(
                    "col",
                    Expr::Lit(Lit::Num(Number {
                        value: col as f64,
                        raw: None,
                        span: DUMMY_SP,
                    })),
                ))),
            ],
            ..Default::default()
        }
    }
}
