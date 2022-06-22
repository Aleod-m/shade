use std::collections::HashMap;



use super::{Op, Expr, Value, Ident};


#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub op: Op,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value {
        match self.op {
            Op::Add => self.expr.eval(context),
            Op::Sub => self.expr.eval(context).negate(),
            _ => panic!("A non unary operator was placed in a unary expr!"),
        }
    }
}

