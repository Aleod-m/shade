use std::fmt;

use crate::utils::*;

use phf::phf_map;
pub use TkKind::*;
pub use Kw::*;
pub use Lit::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TkKind {
    KeyWord(Kw),
    Ident,    // "_"? ASCII_ALPHA*
    Colon,    // :
    Equals,   // =
    Comma,    // ,
    At,       // @
    Dollar,   // $
    Dash,     // -
    GT,       // >
    LT,       // <
    Dot,      // .
    Lpar,     // (
    Rpar,     // )
    Lbrace,   // {
    Rbrace,   // }
    Lbracket, // [
    Rbracket, // ]
    Bar,      // |
    Litteral(Lit), // litteral

    // Unimportant Tokens
    Comment,      // A comment.
    Unrecognized, // None of the above.
}

impl TkKind {
    pub fn is_close_delim(&self) -> bool {
        match self {
            Self::Rpar | Self::Rbracket | Self::Rbrace => true,
            _ => false,
        }
    }

    pub fn is_open_delim(&self) -> bool {
        match self {
            Self::Lpar | Self::Lbracket | Self::Rbrace => true,
            _ => false,
        }
    }

    pub fn get_matching_delim(&self) -> Self {
        match self {
            Self::Rpar => Self::Lpar,
            Self::Lpar => Self::Rpar,
            Self::Rbrace => Self::Lbrace,
            Self::Lbrace => Self::Rbrace,
            Self::Rbracket => Self::Lbracket,
            Self::Lbracket => Self::Rbracket,
            Self::Bar => Self::Bar,
            _ => unreachable!("Caled get_matching_delim on a token that wasn't a delimiter.")
        }
    }
}

impl fmt::Display for TkKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TkKind::*;
        match self {
            Colon    => write!(f, ":"),
            Dash     => write!(f, "-"),
            LT       => write!(f, "<"),
            GT       => write!(f, ">"),
            Lpar     => write!(f, ")"),
            Rpar     => write!(f, "("),
            At       => write!(f, "@"),
            Dollar   => write!(f, "$"),
            Equals   => write!(f, "="),
            Lbrace   => write!(f, "{{"),
            Rbrace   => write!(f, "}}"),
            Lbracket => write!(f, "["),
            Rbracket => write!(f, "]"),
            Comma    => write!(f, ","),
            Dot      => write!(f, "."),
            Bar      => write!(f, "|"),
            Ident         => write!(f, "identifier"),
            Litteral(lit) => write!(f, "Lit({lit})"),
            KeyWord(kw)   => write!(f, "Kw({kw})"),
            Comment       => write!(f, "comment"),
            Unrecognized  => write!(f, "unrecognized"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kw {
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

pub static KEYWORD_MAP: phf::Map<&'static str, Kw> = phf_map! {
    "if" => If,
    "then" => Then,
    "else" => Else,
    "match" => Match,
    "with" => With,
    "let" => Let,
    "in" => In,
};

impl fmt::Display for Kw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            If => write!(f, "if"),
            Then => write!(f, "then"),
            Else => write!(f, "else"),
            Match => write!(f, "match"),
            With => write!(f, "with"),
            Let => write!(f, "let"),
            In => write!(f, "in"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Lit {
    Int,
    Float,
    StringLit,
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Lit::*;
        match self {
            Int => write!(f, "int"),
            Float => write!(f, "float"),
            StringLit => write!(f, "string"),
        }
    }
}
