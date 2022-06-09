use crate::ast;
use crate::lexer::{TokenKind, Lexer};

pub struct Parser;

impl Parser {
    pub fn parse(lexer: Lexer) {
        todo!();
    }
    
    pub fn parse_value(lexer: Lexer) -> Option<ast::Value> {
        let pos = lexer.pos();
        let tok = lexer.next_tok();
        use TokenKind::*;
        match tok.kind {
            Float => Some(ast::Value::Float(tok.text.parse::<f32>().unwrap())),
            Int => Some(ast::Value::Int(tok.text.parse::<i32>().unwrap())),
            _ => {
                lexer.reset_pos(pos);
                None
            }
        }
    }
    
    pub fn parse_unary_expr(lexer: Lexer) -> Option<ast::UnaryExpr> {
    }
}

