use core::fmt;


use crate::lexer::TokenKind;
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Op {
    pub fn is_bin_op(&self) -> bool {
        use Op::*;
        if let Add | Sub | Mul | Div | Mod = self {
            true
        } else {
            false
        }
    }

    pub fn is_unary_op(&self) -> bool {
        use Op::*;
        match self {
            Add | Sub => true,
            _ => false,
        }
    }

    pub fn from_token_kind(kind: TokenKind) -> Option<Self> {
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
            Mul | Div | Mod => 2,
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

