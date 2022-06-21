use std::{iter::Peekable, str::Chars};

use err_derive::Error;

mod token;
pub use token::*;

#[derive(Debug, Error)]
pub enum LexError {
    #[error(display = "Unrecognized token at {}: {}", _0, _1)]
    UnrecognizedToken(Loc, Token),
    #[error(display = "End of input reached.")]
    EOIReached,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    file_path: Option<String>,
    input: Peekable<Chars<'a>>,
    row: usize,
    col: usize,
    ended: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>, file_path: Option<String>) -> Self {
        Self {
            input: chars.peekable(),
            file_path,
            row: 0usize,
            col: 0usize,
            ended: false,
        }
    }

    pub fn loc(&self) -> Loc {
        Loc {
            file_path: self.file_path.clone(),
            row: self.row,
            col: self.col,
        }
    }

    fn trim_whitespaces(&mut self) {
        while let Some(_) = self.input.next_if(|c| {
            match c {
                _ if c.is_whitespace() && *c != '\n' => {self.col += 1; true},
                '\n' => {
                    self.col = 0;
                    self.row += 1;
                    true
                }
                _ => false,
            }
        }) {}
    }

    fn advance(&mut self) -> Option<char> {
        self.col += 1;
        self.input.next()
    }

    fn advance_if(&mut self,f: impl FnOnce(&char)->bool ) -> Option<char> {
        let output = self.input.next_if(f);
        if output.is_some() {
            self.col+=1;
        }
        output
    }

    pub fn lex(&mut self) -> Result<Token, LexError> {
        if self.ended {
            return Err(LexError::EOIReached);
        }
        self.trim_whitespaces();
        let loc = self.loc();
        if let Some(c) = self.advance() {
            let mut text = c.to_string();
            match c {
                ':' => Ok(Token {
                    text,
                    kind: TokenKind::Colon,
                    loc,
                }),
                '+' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Plus,
                }),
                '-' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Dash,
                }),
                '*' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Star,
                }),
                '/' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Slash,
                }),
                '%' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Percent,
                }),
                '(' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Lpar,
                }),
                ')' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::Rpar,
                }),
                '[' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::LBracket,
                }),
                ']' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::RBracket,
                }),
                '{' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::LBrace,
                }),
                '}' => Ok(Token {
                    text,
                    loc,
                    kind: TokenKind::RBrace,
                }),
                _ if c.is_alphabetic() || c == '_' => {
                    while let Some(x) = self.advance_if(is_ident_char) {
                        text.push(x);
                    }
                    Ok(Token {
                        text,
                        loc,
                        kind: TokenKind::Ident,
                    })
                }
                _ if c.is_numeric() => {
                    let mut kind = TokenKind::Int;
                    while let Some(x) = self.advance_if(is_number_char) {
                        text.push(x);
                        if "eE.".contains(x) {
                            kind = TokenKind::Float;
                        };
                    }
                    Ok(Token { kind, text, loc })
                }
                _ => {
                    let unexpected = Token {
                        text,
                        loc: loc.clone(),
                        kind: TokenKind::Unrecognized,
                    };
                    return Err(LexError::UnrecognizedToken(loc, unexpected));
                }
            }
        } else {
            self.col += 1;
            let eoitok = Token {
                kind: TokenKind::EOI,
                loc: self.loc(),
                text: String::default(),
            };
            self.ended = true;
            Ok(eoitok)
        }
    }
}

fn is_ident_char(x: &char) -> bool {
    x.is_alphanumeric() || *x == '_'
}

fn is_number_char(x: &char) -> bool {
    let chars = "eE.";
    x.is_ascii_digit() || chars.contains(*x)
}
