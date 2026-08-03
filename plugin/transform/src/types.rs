use swc_core::ecma::ast::{Expr, ExprOrSpread, KeyValueProp, ObjectLit, Prop, PropName};

fn key_value(key: &str, value: Expr) -> Prop {
    Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(key.into()),
        value: Box::new(value),
    })
}

pub(crate) struct DbgArg {
    /// Stringified actual expression in source code.
    expr: String,
    /// Original expression
    orig: Expr,
}

impl DbgArg {
    pub(crate) const fn new(expr: String, orig: Expr) -> Self {
        Self { expr, orig }
    }

    /// Convert `DbgArg` to `ExprOrSpread` AST.
    ///
    /// ```js
    /// dbg(1, 2, 3);
    ///
    /// // Into
    /// {
    ///     expr: "1, 2, 3",
    ///     value: {
    ///         expr: "1, 2, 3",
    ///         value: 1,
    ///     },
    /// }
    pub(crate) fn into_arg(self) -> ExprOrSpread {
        let Self { expr, orig } = self;

        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Object(ObjectLit {
                props: vec![
                    key_value("expr", Expr::from(expr)).into(),
                    key_value("value", orig).into(),
                ],
                ..Default::default()
            })),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct Loc {
    file: String,
    line: usize,
    column: usize,
}

#[cfg(target_arch = "wasm32")]
impl Loc {
    pub(crate) const fn new(file: String, line: usize, column: usize) -> Self {
        Self { file, line, column }
    }

    /// Convert `Loc` to `ObjectLit` AST.
    ///
    /// ```js
    /// {
    ///     file: "...",
    ///     line: 1,
    ///     col: 1,
    /// }
    /// ```
    pub(crate) fn into_obj_lit(self) -> ObjectLit {
        let Self { file, line, column } = self;

        ObjectLit {
            props: vec![
                key_value("file", Expr::from(file)).into(),
                key_value("line", Expr::from(line)).into(),
                key_value("col", Expr::from(column)).into(),
            ],
            ..Default::default()
        }
    }
}
