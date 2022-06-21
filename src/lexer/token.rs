use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Loc {
    pub file_path: Option<String>,
    pub row: usize,
    pub col: usize,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.file_path {
            Some(fp) => write!(f, "{}:({}, {})", fp, self.row, self.col),
            None => write!(f, "({}, {})", self.row, self.col),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenKind {
    Ident,

    Colon,
    Comma,

    Int,
    Float,

    Plus,
    Dash,
    Star,
    Slash,
    Percent,

    Lpar,
    Rpar,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Unrecognized,
    EOI,
}

impl TokenKind {
    pub fn is_int_op(&self) -> bool {
        use TokenKind::*;
        match self {
            Plus | Dash | Star | Slash | Percent => true,
            _ => false,
        }
    }

    pub fn is_float_op(&self) -> bool {
        use TokenKind::*;
        match self {
            Plus | Dash | Star | Slash => true,
            _ => false,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenKind::*;
        match self {
            Ident => write!(f, "identifier"),
            Colon => write!(f, "colon"),
            Comma => write!(f, "comma"),
            Unrecognized => write!(f, "unrecognized"),
            EOI => write!(f, "end of input"),
            Int => write!(f, "integer"),
            Float => write!(f, "float"),
            Plus => write!(f, "plus"),
            Dash => write!(f, "dash"),
            Star => write!(f, "star"),
            Slash => write!(f, "slash"),
            Percent => write!(f, "percent"),
            Lpar => write!(f, "left parenthesis"),
            Rpar => write!(f, "right parenthesis"),
            LBracket => write!(f, "left bracket"),
            RBracket => write!(f, "right bracket"),
            LBrace => write!(f, "left brace"),
            RBrace => write!(f, "right brace"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub loc: Loc,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.text.is_empty() {
            write!(f, "Token {}: {}", self.kind, self.text)
        } else {
            write!(f, "Token {}", self.kind)
        }
    }
}
