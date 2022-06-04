use std::collections::HashMap;

use crate::{expr::Expr, lexer::Lexer};


pub struct Module {
    exprs: HashMap<String, Box<Expr>>,
}

impl Module {
    
    pub fn new() -> Self {
        Self {
            exprs: HashMap::new() 
        }
    }

    pub fn parse(lexer: &mut Lexer<impl Iterator<Item = char>>) -> Self {
        Self { exprs: HashMap::new() }
    }


}
