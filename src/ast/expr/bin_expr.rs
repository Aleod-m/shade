use std::fmt;

use crate::lexer::TokenKind;


#[derive(Debug, Clone)]
pub enum Expr {
    Binary(ExprBinary),
}

#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl BinOp {
    fn from_token_kind(kind: TokenKind) -> Option<Self> {
        use TokenKind::*;
        match kind {
            Plus => Some(BinOp::Add),
            Dash => Some(BinOp::Sub),
            Star => Some(BinOp::Mul),
            Slash => Some(BinOp::Div),
            Percent => Some(BinOp::Mod),
            _ => None,
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Mod => write!(f, "%"),
        }
    }
}
