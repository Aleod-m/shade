use core::fmt;
use std::fmt::format;

use err_derive::Error;

use crate::{lexer::{TokenKind, Token, Lexer}};


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
pub fn parse_op(lexer: Lexer) -> Option<Op> {
    let pos = lexer.pos();
    let tok = Op::from_token_kind(lexer.peek_tok().kind);
    if tok.is_none() {
        lexer.reset_pos(pos);
    }
    tok
}