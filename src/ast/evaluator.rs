use std::collections::HashMap;

use super::{Expr, Value, Stmt, Ident};

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    TypeError(String),
    ValueNotPresentInContext(String),
}

pub struct Evaluator {
    context: HashMap<Ident, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Value {
        expr.eval(&mut self.context)
    }
    
    pub fn eval_stmt(&mut self, stmt: Stmt) -> Value  {
        stmt.eval(&mut self.context)
    }
    
    pub fn get_val_in_ctx(&self, ident: Ident) -> Option<&Value> {
        self.context.get(&ident) 
    }
}

#[cfg(test)]
pub mod test {
    use crate::ast::{Value, Expr, Op, Stmt};

    use super::Evaluator;

    #[test]
    fn eval_of_negation() {
        let int = Expr::unary(Op::Sub, Expr::value(16i32));
        let float = Expr::unary(Op::Sub, Expr::value(139f32));

        let mut ev = Evaluator::new();
        assert_eq!(ev.eval_expr(int), Value::new(-16i32));
        assert_eq!(ev.eval_expr(float), Value::new(-139f32));
    }

    #[test]
    fn eval_statement() {
        let stmt = Stmt::assign("a".to_string().into(), Expr::value(122i32));
        let mut ev = Evaluator::new();
        let _ = ev.eval_stmt(stmt);
        assert_eq!(ev.get_val_in_ctx("a".to_string().into()), Some(&Value::new(122i32)));
    }
}
