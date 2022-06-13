use std::collections::HashMap;

use crate::types::{Type, TypeCheckVisitor, TypeError};

use super::{Expr, Op, Value};

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: Op,
    pub rhs: Box<Expr>,
}

impl BinaryExpr {
    pub fn eval(&self, context: &mut HashMap<String, Value>) -> Value  {
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

impl TypeCheckVisitor for BinaryExpr {
    fn ty_check(&self, context: &mut HashMap<String, Type>) -> Result<Type, TypeError> {
        let lty = self.lhs.ty_check(context)?;
        use Op::*;
        match self.op {
            Add | Sub | Mul | Div | Mod => {
                if lty == self.rhs.ty_check(context)? {
                    Ok(lty)
                } else {
                    Err(TypeError::OperatorError)
                }
            }
        }
    }
}
