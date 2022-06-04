use core::fmt;
use std::fmt::format;

use crate::{lexer::{TokenKind, Token, Lexer}, types::Type};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Op {
    fn from_token_kind(kind: TokenKind) -> Option<Self> {
        use TokenKind::*;
        match kind {
            Plus => Some(Op::Add),
            Dash => Some(Op::Sub),
            Star => Some(Op::Mul),
            Slash => Some(Op::Div),
            Percent => Some(Op::Mod),
            _ => None,
        }
    }

    fn prec(&self) -> usize {
        use Op::*;
        match self {
            Add | Sub => 1,
            Mul | Div | Mod=> 2,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Op::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Mod => write!(f, "%"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Fun{ident: String, body: Box<Expr>},
    Op{op: Op, lhs: Box<Expr>, rhs: Box<Expr>},
    Const{ty: Type, val: String},
}

pub enum ExprError {
    ParseError(String),
}

impl Expr {

    pub fn eval(&self) {
        match self {
            Self::Fun { ident, body } => todo!(),
            Expr::Fun { ident, body } => todo!(),
            Expr::Op { op, lhs, rhs } => todo!(),
            Expr::Const { ty, val } => todo!(),
        }
    }

    pub fn parse_func(&self, lexer: &mut Lexer<impl Iterator<Item = char>>) -> Result<Self, ExprError> {
        use TokenKind::*;
        let ident : String;
        let body: Box<Expr>;
        let token = lexer.expect_tok(Ident);
        match token {
            Ok(tok) => ident = tok.text,
            Err(tok) => return Err(ExprError::ParseError("Unexpected Token: {}".to_string())),
        }
        if lexer.peek_tok().kind != Colon {
            return Err(ExprError::ParseError("Expected token".to_string()));
        } else {
            lexer.next_tok();
        }
        body = Box::new(self.parse_expr(lexer)?);
        Ok(Self::Fun {
            ident,
            body,
        })
    }

    pub fn parse_expr(&self, lexer: &mut Lexer<impl Iterator<Item = char>>) -> Result<Self, ExprError> {
        use TokenKind::*;
        match lexer.peek_tok().kind {
            Int => {
                let int = Self::Const { ty: Type::Int, val: lexer.next_tok().text };
                if lexer.peek_tok().kind.is_int_op() {
                    let op_tok = lexer.next_tok();
                    let rhs = Box::new(self.parse_expr(lexer)?); 
                    Ok(Self::Op { op: Op::from_token_kind(lexer.peek_tok().kind).unwrap(), lhs: Box::new(int), rhs })
                } else {
                    Ok(int)
                }
            },
            Float => {
                let float = Self::Const { ty: Type::Float, val: lexer.next_tok().text };
                if lexer.peek_tok().kind.is_float_op() {
                    let op_tok = lexer.next_tok();
                    let rhs = Box::new(self.parse_expr(lexer)?); 
                    Ok(Self::Op { op: Op::from_token_kind(lexer.peek_tok().kind).unwrap(), lhs: Box::new(float), rhs })
                } else {
                    Ok(float)
                }
            },
            _ => Err(ExprError::ParseError("not implemented".to_string())),
        }

    }

}
