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

pub struct Pos(pub usize, pub usize);

impl Pos {
    fn num_prop(&self, key: &str, value: usize) -> Prop {
        Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(IdentName {
                span: DUMMY_SP,
                sym: key.into(),
            }),
            value: Box::new(Expr::Lit(Lit::Num(Number {
                value: value as f64,
                span: DUMMY_SP,
                raw: None,
            }))),
        })
    }

    pub fn into_obj_lit(self) -> ObjectLit {
        let Self(line, col) = self;

        ObjectLit {
            span: DUMMY_SP,
            props: vec![
                PropOrSpread::Prop(Box::new(self.num_prop("line", line))),
                PropOrSpread::Prop(Box::new(self.num_prop("col", col))),
            ],
            ..Default::default()
        }
    }
}
