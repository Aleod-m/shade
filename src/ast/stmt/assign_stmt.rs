use std::collections::HashMap;

use crate::ast::{Expr, Value, Ident};

pub struct AssignStmt {
    pub ident: Ident,
    pub expr: Expr,
}

impl AssignStmt {
    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value {
        let value = self.expr.eval(context);
        context.insert(self.ident.clone(), value);
        Value::new(())
    }
}
