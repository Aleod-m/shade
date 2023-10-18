use std::collections::HashMap;

use crate::ast::{Value, Ident, Function};

#[derive(Debug)]
pub struct FunctionDef {
    pub ident: Ident,
    pub fun: Function,
}

impl FunctionDef {
    
    pub fn new(ident: Ident, fun: Function) -> Self {
        Self {
            ident,
            fun,
        }
    }

    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value {
        let value = self.fun.eval(context);
        context.insert(self.ident.clone(), value);
        Value::new(())
    }
}
