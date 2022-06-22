use std::collections::HashMap;


use super::{Expr, Op, Value, Ident};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
}

impl BinaryExpr {
    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value  {
        let lhs = self.lhs.eval(context);
        let rhs = self.rhs.eval(context);
        match self.op {
            Op::Add => lhs.add(rhs),
            Op::Sub => lhs.sub(rhs),
            Op::Mod => lhs.modulo(rhs),
            Op::Mul => lhs.mul(rhs),
            Op::Div => lhs.div(rhs),
            _ => unreachable!(),
        }
    }
}

