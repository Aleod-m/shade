use std::collections::HashMap;

use super::{Expr, Value};

pub struct Evaluator {
    context: HashMap<Ident, Value>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            context: HashMap::new(),
        }
    }

    pub fn eval(expr: )
}
