use super::Ident;

mod op;
pub use op::*;
mod unary_expr;
pub use unary_expr::*;
mod bin_expr;
pub use bin_expr::*;
mod value;
pub use value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Value(Value),
    Ident(Ident),
}

impl Expr {
    pub fn unary(op: Op, expr: Expr) -> Self {
        if !op.is_unary_op() {
            panic!("Op is not unary!");
        }
        Expr::Unary(UnaryExpr {
            op,
            expr: Box::new(expr),
        })
    }

    pub fn binary(op: Op, lhs: Expr, rhs: Expr) -> Self {
        if !op.is_bin_op() {
            panic!("Op is not unary!");
        }
        Expr::Binary(BinaryExpr {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    pub fn value(val: impl Into<Value>) -> Self {
        Self::Value(val.into())
    }
}
