use std::collections::HashMap;

use crate::ast::{Expr, EvaluationError, Value};

pub struct AssignStmt {
    pub ident: String,
    pub expr: Expr,
}

impl AssignStmt {
    pub fn eval(&self, context: &mut HashMap<String, Value>) -> Value {
        let value = self.expr.eval(context);
        context.insert(self.ident.to_string(), value);
        Value::new(())
    }

}
