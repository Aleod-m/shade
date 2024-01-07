use crate::utils::*;

mod input;
pub mod token;
use thiserror::Error;
use token::*;

use input::Cursor;

pub type TkHandle = usize;

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unbalanced delims.")]
    UnbalancedDelimiter(Loc),
}
pub type Result<T> = std::result::Result<T, (LexedBuffer, LexError)>;

/// Output of the lexing stage.
#[derive(Debug)]
pub struct LexedBuffer {
    kinds: IVec<TkKind>,
    spans: IVec<Span>,
}

impl LexedBuffer {
    pub fn nb_tokens(&self) -> usize {
        self.kinds.len()
    }

    pub fn next_handle(&self, handle: TkHandle) -> Option<TkHandle> {
        if handle >= self.kinds.len() {
            return None;
        } 
        Some(handle + 1)
    }

    pub fn first(&self) -> Option<TkHandle> {
        if self.kinds.len() == 0 {
            None
        } else {
            Some(0usize)
        }
    }

    pub fn get_kind(&self, id: TkHandle) -> &TkKind {
        &self.kinds[id]
    }

    pub fn get_token_txt(&self, id: TkHandle, input: IStr) -> IStr {
        let span = self.spans[id];
        let range: std::ops::Range<_> = span.into();
        input[range].into()
    }
}

/// Lexing state. Call new with an input to create, and lex to lex the input.
#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    tokens: Vec<TkKind>,
    spans: Vec<Span>,
    start: Loc,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let cursor = Cursor::new(input);
        let start = cursor.loc();
        Self {
            cursor,
            tokens: vec![],
            spans: vec![],
            start,
        }
    }

    pub fn lex(mut self) -> Result<LexedBuffer> {
        self.trim_whitespaces();
        while let Some(c) = self.cursor.next() {
            match c {
                ':' => self.push_token(TkKind::Colon)?,
                '=' => self.push_token(TkKind::Equals)?,
                '(' => self.push_token(TkKind::Lpar)?,
                ')' => self.push_token(TkKind::Rpar)?,
                '@' => self.push_token(TkKind::At)?,
                '-' => self.resolve_comment()?,
                '_' => self.lex_identifier(),
                _ if c.is_alphabetic() || c == '_' => self.lex_ident_or_keyword(c),
                '0' if self.cursor.next_if(|c| *c == 'b').is_some() => self.lex_bin_int(),
                '0' if self.cursor.next_if(|c| *c == 'x').is_some() => self.lex_hex_int(),
                _ if c.is_numeric() => self.lex_float_or_int(),
                _ => self.push_token(TkKind::Unrecognized)?,
            };
            self.trim_whitespaces();
        }
        Ok(self.finalize())
    }

    fn finalize(self) -> LexedBuffer {
        LexedBuffer {
            kinds: self.tokens.into(),
            spans: self.spans.into(),
        }
    }

    fn push_token(&mut self, kind: TkKind) -> Result<()> {
        self.tokens.push(kind);
        // TODO Handle balanced delimiters.
        self.spans.push(self.cursor.span(self.start));
        self.start = self.cursor.loc();
        Ok(())
    }

    fn trim_whitespaces(&mut self) {
        while let Some(_) = self.cursor.next_if(|c| c.is_whitespace()) {}
    }

    fn lex_identifier(&mut self) {
        self.cursor.skip_while(pred::ident_char);
        self.push_token(TkKind::Ident).unwrap()
    }

    fn lex_ident_or_keyword(&mut self, c: char) {
        let mut kw = String::from(c);
        while let Some(c) = self.cursor.next_if(pred::ident_char) {
            kw.push(c);
        }
        if let Some(kw) = KEYWORD_MAP.get(&kw) {
            self.push_token(TkKind::KeyWord(*kw)).unwrap()
        } else {
            self.push_token(TkKind::Ident).unwrap()
        }
    }

    fn lex_float_or_int(&mut self) {
        let mut is_float = false;
        while let Some(x) = self.cursor.next_if(pred::number_char) {
            if "eE.".contains(x) {
                is_float = true;
                self.cursor.skip_if(|c| "-+".contains(*c) && x != '.');
            };
        }
        self.push_token(TkKind::Litteral);
    }

    fn lex_bin_int(&mut self) {
        self.cursor.skip_while(pred::bin_int_char);
        self.push_token(TkKind::Litteral);
    }

    fn lex_hex_int(&mut self) {
        self.cursor.skip_while(pred::bin_int_char);
        self.push_token(TkKind::Litteral);
    }

    fn resolve_comment(&mut self) -> Result<()> {
        match self.cursor.peek_char() {
            Some('-') => {
                // a single line comment '--'
                self.cursor.skip_while(|c| *c != '\n');
                self.push_token(TkKind::Comment).unwrap();
            }
            Some('{') => loop {
                // Multiline comment '-{ ... }-'.
                self.cursor.skip_while(|c| *c != '}');
                if let Some('-') = self.cursor.peek_char() {
                    self.cursor.next();
                    self.push_token(TkKind::Comment);
                    break;
                }
            },
            _ => self.push_token(TkKind::Unrecognized)?,
        }
        Ok(())
    }
}

mod pred {
    pub fn ident_char(x: &char) -> bool {
        x.is_alphanumeric() || *x == '_'
    }

    pub fn number_char(x: &char) -> bool {
        let chars = "eE.";
        x.is_ascii_digit() || chars.contains(*x)
    }

    pub fn hexa_int_char(x: &char) -> bool {
        x.is_ascii_digit() || "aAbBcCdDeEfF".contains(*x)
    }

    pub fn bin_int_char(x: &char) -> bool {
        "01".contains(*x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn match_kinds(found: IVec<TkKind>, expected: Vec<TkKind>) -> bool {
        for (found, expected) in found.iter().zip(expected.iter()) {
            if found != expected {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn test_basic_lexing() {
        let input: IStr = "comp = f: g: x: f (g x)".into();
        let lexed = Lexer::new(&input).lex().unwrap();
        match_kinds(
            lexed.kinds,
            vec![
                TkKind::Ident, // comp
                TkKind::Equals,// =
                TkKind::Ident, // f
                TkKind::Colon, // :
                TkKind::Ident, // g
                TkKind::Colon, // :
                TkKind::Ident, // x
                TkKind::Colon, // :
                TkKind::Ident, // f
                TkKind::Lpar,  // (
                TkKind::Ident, // g
                TkKind::Ident, // x
                TkKind::Rpar,  // )
            ],
        );
    }
}
