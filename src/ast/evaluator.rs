use std::collections::HashMap;

use super::{Expr, Value};

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    TypeError(String),
    ValueNotPresentInContext(String),
}

pub struct Evaluator {
    context: HashMap<String, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: Expr) -> Result<Value, EvaluationError> {
        expr.eval(&mut self.context)
    }
}

#[cfg(test)]
pub mod test {
    use crate::ast::{Value, Expr, Op};

    use super::Evaluator;

    #[test]
    fn eval_of_negation() {
        let int = Expr::unary(Op::Sub, Expr::Value(Value::Int(16i32)));
        let float = Expr::unary(Op::Sub, Expr::Value(Value::Float(139f32)));

        let mut ev = Evaluator::new();
        assert_eq!(ev.eval(int), Ok(Value::Int(-16i32)));
        assert_eq!(ev.eval(float), Ok(Value::Float(-139f32)));
    }
}
