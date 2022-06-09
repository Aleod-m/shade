use std::fmt;
use std::iter::Peekable;

use err_derive::Error;

#[derive(Debug, Error)]
enum LexError {
    #[error(display = "Unrecognized token at {}: {}", _0, _1)]
    UnrecognizedToken(String, Loc),
}

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

    Int,
    Float,

    Plus,
    Dash,
    Star,
    Slash,
    Percent,

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
            Unrecognized => write!(f, "unrecognized"),
            EOI => write!(f, "end of input"),
            Int => write!(f, "integer"),
            Float => write!(f, "float"),
            Plus => write!(f, "plus"),
            Dash => write!(f, "dash"),
            Star => write!(f, "star"),
            Slash => write!(f, "slash"),
            Percent => write!(f, "percent"),
        }
    }
}

#[derive(Debug, PartialEq)]
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

pub struct Lexer {
    file_path: Option<String>,
    peeked: Option<Token>,
    input: Vec<char>,
    idx: usize,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(chars: impl Iterator<Item = char>, file_path: Option<String>) -> Self {
        Self {
            input: chars.collect(),
            peeked: None,
            idx: 0usize,
            file_path,
            row: 0usize,
            col: 0usize,
        }
    }

    pub fn loc(&self) -> Loc {
        Loc {
            file_path: self.file_path.clone(),
            row: self.row,
            col: self.col,
        }
    }

    pub fn expect_tok(&mut self, kind: TokenKind) -> bool {
        self.lex().kind == kind
    }

    pub fn next_tok(&mut self) -> Token {
        self.lex()
    }

    pub fn peek_tok(&mut self) -> &Token {
        let tok = self.lex();
        self.peeked = Some(tok);
        &tok
    }

    fn trim_whitespaces(&mut self) {
        let stop = false;
        while !stop
        {
            let c = self.input[self.idx];
            match c {
                _ if c.is_whitespace() && c != '\n' => self.col += 1,
                '\n' => stop = true,
            }
            self.idx += 1;
            stop |= self.idx == self.input.len();
        }
    }

    fn lex(&mut self) -> Token {
        self.trim_whitespaces();
        let loc = self.loc();
        if let Some(x) = self.input[self.idx] {
            self.col += 1;
            let mut text = x.to_string();
            match x {
                ':' => Token {
                    text,
                    loc,
                    kind: TokenKind::Colon,
                },
                '+' => Token {
                    text,
                    loc,
                    kind: TokenKind::Plus,
                },
                '-' => Token {
                    text,
                    loc,
                    kind: TokenKind::Dash,
                },
                '*' => Token {
                    text,
                    loc,
                    kind: TokenKind::Star,
                },
                '/' => Token {
                    text,
                    loc,
                    kind: TokenKind::Slash,
                },
                '%' => Token {
                    text,
                    loc,
                    kind: TokenKind::Percent,
                },
                _ if x.is_alphabetic() || x == '_' => {
                    while let Some(x) = self.input.next_if(is_ident_char) {
                        self.col += 1;
                        text.push(x);
                    }
                    Token {
                        text,
                        loc,
                        kind: TokenKind::Ident,
                    }
                }
                _ if x.is_numeric() => {
                    let mut kind = TokenKind::Int;
                    while let Some(x) = self.input.next_if(is_number_char) {
                        self.col += 1;
                        text.push(x);
                        if "eE.".contains(x) {
                            kind = TokenKind::Float;
                        };
                    }
                    Token { kind, text, loc }
                }
                _ => Token {
                    text,
                    loc,
                    kind: TokenKind::Unrecognized,
                },
            }
        } else {
            self.col += 1;
            let eoitok = Token {
                kind: TokenKind::EOI,
                loc: Loc::default(),
                text: String::default(),
            };
            eoitok
        }
    }

    pub fn pos(&self) -> (usize, usize, usize) {
        (self.row, self.col, self.idx)
    }

    pub fn reset_pos(&mut self, mark: (usize, usize, usize)) {
        self.peeked = None;
        (self.row, self.col, self.idx) = mark;
    }

    pub fn is_empty(&self) -> bool {
        self.idx == self.input.len()
    }
}

fn is_ident_char(x: &char) -> bool {
    x.is_alphanumeric() || *x == '_'
}

fn is_number_char(x: &char) -> bool {
    let chars = "eE+-.";
    x.is_numeric() || chars.contains(*x)
}
