use super::{Op, Expr};

pub struct UnaryExpr {
    op: Op,
    expr: Box<Expr>,
}
