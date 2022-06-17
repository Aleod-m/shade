use err_derive::Error;

mod token;
pub use token::*;

#[derive(Debug, Error)]
enum LexError {
    #[error(display = "Unrecognized token at {}: {}", _0, _1)]
    UnrecognizedToken(String, Loc),
}

#[derive(Debug)]
pub struct Lexer {
    file_path: Option<String>,
    input: Vec<char>,
    idx: usize,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(chars: impl Iterator<Item = char>, file_path: Option<String>) -> Self {
        Self {
            input: chars.collect(),
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

    fn trim_whitespaces(&mut self) {
        while !self.is_empty() {
            let c = self.get_char();
            match c {
                _ if c.is_whitespace() && c != '\n' => self.col += 1,
                '\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                _ => break,
            }
            self.idx += 1;
        }
    }

    fn advance(&mut self) -> char {
        let c = self.get_char();
        self.idx += 1;
        return c;
    }

    fn get_char(&self) -> char {
        self.input[self.idx]
    }

    pub fn lex(&mut self) -> Token {
        self.trim_whitespaces();
        let loc = self.loc();
        if !self.is_empty() {
            let mut c = self.advance();
            let mut text = c.to_string();
            match c {
                ':' => Token {
                    text,
                    kind: TokenKind::Colon,
                    loc,
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
                '(' => Token {
                    text,
                    loc,
                    kind: TokenKind::Lpar,
                },
                ')' => Token {
                    text,
                    loc,
                    kind: TokenKind::Rpar,
                },
                '[' => Token {
                    text,
                    loc,
                    kind: TokenKind::LBracket,
                },
                ']' => Token {
                    text,
                    loc,
                    kind: TokenKind::RBracket,
                },
                '{' => Token {
                    text,
                    loc,
                    kind: TokenKind::LBrace,
                },
                '}' => Token {
                    text,
                    loc,
                    kind: TokenKind::RBrace,
                },
                _ if c.is_alphabetic() || c == '_' => {
                    let mut c = self.advance();
                    while is_ident_char(&c) {
                        text.push(c);
                        if self.is_empty() {
                            break;
                        }
                        c = self.advance();
                    }
                    Token {
                        text,
                        loc,
                        kind: TokenKind::Ident,
                    }
                }
                _ if c.is_numeric() => {
                    let mut kind = TokenKind::Int;
                    c = self.advance();
                    while is_number_char(&c) {
                        text.push(c);
                        if self.is_empty() {
                            break;
                        }
                        if "eE.".contains(c) {
                            kind = TokenKind::Float;
                        };
                        c = self.advance();
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
                loc: self.loc(),
                text: String::default(),
            };
            eoitok
        }
    }

    pub fn pos(&self) -> (usize, usize, usize) {
        (self.row, self.col, self.idx)
    }

    pub fn reset_pos(&mut self, mark: (usize, usize, usize)) {
        (self.row, self.col, self.idx) = mark;
    }

    pub fn is_empty(&self) -> bool {
        self.idx >= self.input.len() - 1
    }
}

fn is_ident_char(x: &char) -> bool {
    x.is_alphanumeric() || *x == '_'
}

fn is_number_char(x: &char) -> bool {
    let chars = "eE.";
    x.is_ascii_digit() || chars.contains(*x)
}
