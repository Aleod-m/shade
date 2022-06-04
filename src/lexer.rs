use std::fmt;
use std::iter::Peekable;

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
    SOI,
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
            SOI => write!(f, "start of input"),
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

pub struct Lexer<C: Iterator<Item = char>> {
    chars: Peekable<C>,
    peeked: Option<Token>,
    exhausted: bool,
    file_path: Option<String>,
    lnum: usize,
    cnum: usize,
}

impl<C: Iterator<Item = char>> Lexer<C> {
    pub fn new(chars: C, file_path: Option<String>) -> Self {
        let soitok = Token {
            kind: TokenKind::SOI,
            loc: Loc::default(),
            text: String::default(),
        };
        Self {
            chars: chars.peekable(),
            peeked: Some(soitok),
            exhausted: false,
            file_path,
            lnum: 0usize,
            cnum: 0usize,
        }
    }

    pub fn loc(&self) -> Loc {
        Loc {
            file_path: self.file_path.clone(),
            row: self.lnum,
            col: self.cnum,
        }
    }

    pub fn next_tok(&mut self) -> Token {
        self.peeked.take().unwrap_or_else(|| self.lex())
    }

    pub fn peek_tok(&mut self) -> &Token {
        let token = self.next_tok();
        self.peeked.insert(token)
    }

    pub fn expect_tok(&mut self, kind: TokenKind) -> Result<Token, Token> {
        let token = self.next_tok();
        if kind == token.kind {
            Ok(token)
        } else {
            Err(token)
        }
    }

    fn trim_whitespaces(&mut self) {
        while self
            .chars
            .next_if(|c| c.is_whitespace() && *c != '\n')
            .is_some()
        {
            self.cnum += 1;
        }
    }

    fn lex(&mut self) -> Token {
        self.trim_whitespaces();
        let loc = self.loc();
        if let Some(x) = self.chars.next() {
            self.cnum += 1;
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
                    while let Some(x) = self.chars.next_if(is_ident_char) {
                        self.cnum += 1;
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
                    while let Some(x) = self.chars.next_if(is_number_char) {
                        self.cnum += 1;
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
            self.cnum += 1;
            let eoitok = Token {
                kind: TokenKind::EOI,
                loc: Loc::default(),
                text: String::default(),
            };
            self.exhausted = true;
            eoitok
        }
    }
}

impl<C: Iterator<Item = char>> Iterator for Lexer<C> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            Some(self.next_tok())
        }
    }
}

fn is_ident_char(x: &char) -> bool {
    x.is_alphanumeric() || *x == '_'
}

fn is_number_char(x: &char) -> bool {
    let chars = "eE+-.";
    x.is_numeric() || chars.contains(*x)
}
