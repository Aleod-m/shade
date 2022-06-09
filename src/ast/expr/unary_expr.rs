use std::collections::HashMap;

use crate::ast::EvaluationError;

use super::{Op, Expr, Value};

pub struct UnaryExpr {
    pub op: Op,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn eval(&self, context: &mut HashMap<String, Value>) -> Result<Value, EvaluationError> {
        if !self.op.is_unary_op() {
            return Err(EvaluationError::TypeError("Operator is not unary!".to_string()));
        }

        match self.op {
            Op::Add => self.expr.eval(context),
            Op::Sub => self.expr.eval(context).map(Value::negate),
            _ => unreachable!()
        }
    }


}

