use std::fmt;

use crate::utils::*;

use phf::phf_map;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TkKind {
    KeyWord(KeyWord),
    Ident, // "_"? ASCII_ALPHA*
    Colon, // :
    Equals, // =

    Lpar, // (
    Rpar, // )
    
    At,     // @
    Dollar, // $

    // Litterals
    Litteral,

    // Unimportant Tokens
    Comment,      // A comment.
    Unrecognized, // None of the above.
}

impl TkKind {
    pub fn is_close_delim(&self) -> bool {
        match self {
            Self::Rpar => true,
            _ => false,
        }
    }

    pub fn is_open_delim(&self) -> bool {
        match self {
            Self::Lpar => true,
            _ => false,
        }
    }

    pub fn get_matching_delim(&self) -> Self {
        match self {
            Self::Rpar => Self::Lpar,
            Self::Lpar => Self::Rpar,
            _ => unreachable!("Caled get_matching_delim on a token that wasn't a delimiter.")
        }
    }
}

impl fmt::Display for TkKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TkKind::*;
        match self {
            Colon => write!(f, ":"),
            Lpar => write!(f, ")"),
            Rpar => write!(f, "("),
            At => write!(f, "@"),
            Dollar => write!(f, "$"),
            Equals => write!(f, "="),
            Ident => write!(f, "identifier"),
            Litteral => write!(f, "literal"),
            KeyWord(kw) => write!(f, "keyword: {kw}"),
            Comment => write!(f, "comment"),
            Unrecognized => write!(f, "unrecognized"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyWord {
    // If Control flow.
    If,
    Then,
    Else,

    // Match Control flow.
    Match,
    With,

    // Binding.
    Let,
    In,
}

pub static KEYWORD_MAP: phf::Map<&'static str, KeyWord> = phf_map! {
    "if" => KeyWord::If,
    "then" => KeyWord::Then,
    "else" => KeyWord::Else,
    "match" => KeyWord::Match,
    "with" => KeyWord::With,
    "let" => KeyWord::Let,
    "in" => KeyWord::In,
};

impl fmt::Display for KeyWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyWord::If => write!(f, "if"),
            KeyWord::Then => write!(f, "then"),
            KeyWord::Else => write!(f, "else"),
            KeyWord::Match => write!(f, "match"),
            KeyWord::With => write!(f, "with"),
            KeyWord::Let => write!(f, "let"),
            KeyWord::In => write!(f, "in"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Litteral {
    Int(i64),
    Float(f64),
    StringLit(IStr),
}

impl fmt::Display for Litteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Litteral::Int(i) => write!(f, "int : {i}"),
            Litteral::Float(float) => write!(f, "float : {float}"),
            Litteral::StringLit(s) => write!(f, "string: \"{s}\""),
        }
    }
}
