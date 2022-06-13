use std::collections::HashMap;

use crate::types::{TypeCheckVisitor, Type, TypeError};

use super::{Op, Expr, Value};


#[derive(Debug)]
pub struct UnaryExpr {
    pub op: Op,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    
    pub fn eval(&self, context: &mut HashMap<String, Value>) -> Value {
        match self.op {
            Op::Add => self.expr.eval(context),
            Op::Sub => self.expr.eval(context).negate(),
            _ => panic!("A non unary operator was placed in a unary expr!"),
        }
    }
}

impl TypeCheckVisitor for UnaryExpr {
    fn ty_check(&self, context: &mut HashMap<String, Type>) -> Result<Type, TypeError> {
        self.expr.ty_check(context)
    }
}
