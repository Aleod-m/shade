use std::collections::HashMap;

use super::{Expr, Ident, Op, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub op: Op,
    pub expr: Box<Expr>,
}

