use swc_core::ecma::ast::*;
use utils::kv;

mod utils {
    use super::*;

    /// Create a key-value property AST for the object literal.
    pub fn kv(key: &str, value: Expr) -> Prop {
        Prop::KeyValue(KeyValueProp {
            key: PropName::Ident(key.into()),
            value: Box::new(value),
        })
    }
}

pub struct DbgArg {
    /// Stringified actual expression in source code.
    expr: String,
    /// Original expression
    orig: Expr,
}

impl DbgArg {
    pub fn new(expr: String, orig: Expr) -> Self {
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
    pub fn into_arg(self) -> ExprOrSpread {
        ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Object(ObjectLit {
                props: vec![
                    kv("expr", Expr::from(self.expr)).into(),
                    kv("value", self.orig).into(),
                ],
                ..Default::default()
            })),
        }
    }
}

pub struct Loc(pub String, pub usize, pub usize);

impl Loc {
    /// Convert `Loc` to `ObjectLit` AST.
    ///
    /// ```js
    /// {
    ///     file: "...",
    ///     line: 1,
    ///     col: 1,
    /// }
    /// ```
    pub fn into_obj_lit(self) -> ObjectLit {
        let Self(ref file, line, col) = self;

        ObjectLit {
            props: vec![
                kv("file", Expr::from(file.to_owned())).into(),
                kv("line", Expr::from(line)).into(),
                kv("col", Expr::from(col)).into(),
            ],
            ..Default::default()
        }
    }
}
