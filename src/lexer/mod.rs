use std::vec;

use crate::{utils::*, input::Input};

// TODO: Arrays

pub mod token;
use thiserror::Error;
use token::*;

pub type TkHandle = usize;

use Reason::*;

#[derive(Debug, Error)]
pub enum Reason {
    #[error("Unexpected end of input.")]
    UnexpectedEOI,
    #[error("Unbalanced delims.")]
    UnbalancedDelimiter(Loc),
}
type Result<T> = std::result::Result<T, (LexError, LexedBuffer)>;

pub type LexerInput<'a> = Input<std::str::Chars<'a>, Loc>;

#[derive(Debug)]
pub enum LexMode {
    Default,
}

#[derive(Debug, Error)]
#[error("Lexing error at: {} because {}", loc, reason)]
pub struct LexError {
    loc: Loc,
    reason: Reason,
}

fn lex(input: &str) -> Result<LexedBuffer> {
    let mut state = LexerState::new(input);
    state.trim_whitespaces();
    while let Some(c) = state.input.next() {
        state = match c {
            ':' => state.push_token(Colon),
            '=' => state.push_token(Equals),
            '(' => state.push_token(Lpar),
            ')' => state.push_token(Rpar),
            '{' => state.push_token(Lbrace),
            '}' => state.push_token(Rbrace),
            '[' => state.push_token(Lbracket),
            ']' => state.push_token(Rbracket),
            '|' => state.push_token(Bar),
            '@' => state.push_token(At),
            '$' => state.push_token(Dollar),
            '<' => state.push_token(LT),
            '>' => state.push_token(GT),
            '.' => state.push_token(Dot),
            ',' => state.push_token(Comma),
            '"' => lex_string(state)?,
            '-' => resolve_comment(state)?,
            '_' => lex_identifier(state)?,
            _ if c.is_alphabetic() || c == '_' => lex_ident_or_keyword(state, c)?,
            '0' if state.input.next_is('b') => lex_bin_int(state)?,
            '0' if state.input.next_is('x') => lex_hex_int(state)?,
            _ if c.is_numeric() => lex_float_or_int(state)?,
            _ => state.push_token(Unrecognized),
        };
        state.trim_whitespaces();
    }
    Ok(state.finalize())
}

fn lex_identifier(mut state: LexerState) -> Result<LexerState>{
    state.input.skip_while(pred::ident_char);
    Ok(state.push_token(Ident))
}

fn lex_ident_or_keyword(mut state: LexerState, c: char) -> Result<LexerState> {
    let mut kw = String::from(c);
    while let Some(c) = state.input.next_if(pred::ident_char) {
        kw.push(c);
    }
    state = if let Some(kw) = KEYWORD_MAP.get(&kw) {
        state.push_token(KeyWord(*kw))
    } else {
        state.push_token(Ident)
    };
    Ok(state)
}

fn lex_string(mut state: LexerState) -> Result<LexerState> {
    state = loop {
        state = match state.input.next() {
            None => state.error_out(UnexpectedEOI)?,
            Some('/') if state.input.skip_next('"') => state,
            Some('"') => break state.push_token(Litteral(StringLit)),
            _ => state
        }
    };
    Ok(state)
}

fn lex_float_or_int(mut state: LexerState) -> Result<LexerState> {
    let mut kind = Int;
    while let Some(x) = state.input.next_if(pred::number_char) {
        if "eE.".contains(x) {
            state.input.skip_if(|c| "-+".contains(*c) && x != '.');
        };
        kind = Float;
    }
    Ok(state.push_token(Litteral(kind)))
}

fn lex_bin_int(mut state: LexerState) -> Result<LexerState> {
    state.input.skip_while(pred::bin_int_char);
    Ok(state.push_token(Litteral(Int)))
}

fn lex_hex_int(mut state: LexerState) -> Result<LexerState> {
    state.input.skip_while(pred::hexa_int_char);
    Ok(state.push_token(Litteral(Int)))
}

fn resolve_comment(mut state: LexerState) -> Result<LexerState> {
    state = match state.input.peek() {
        Some('-') => {
            //  single line comment '--'
            state.input.skip_while(|c| *c != '\n');
            state.push_token(Comment)
        }
        Some('{') => loop {
            // Multiline comment '-{ ... }-'.
            eprintln!("multi");
            state.input.skip_while(|c| *c != '}');
            state.input.next();
            if state.input.next_is('-') {
                break state.push_token(Comment);
            }
        },
        _ => state.push_token(Dash),
    };
    Ok(state)
}

/// Lexing state. Call new with an input to create, and lex to lex the input.
#[derive(Debug)]
pub struct LexerState<'a> {
    mode: LexMode,
    source: IStr,
    input: LexerInput<'a>,
    tokens: Vec<TkKind>,
    spans: Vec<Span>,
    delim_stack: Vec<TkKind>,
    start: Loc,
    minor_errors: Vec<LexError>
}

impl<'a> LexerState<'a> {
    pub fn new(input: &'a str) -> LexerState<'a> {
        let source = input.into();
        let input = LexerInput::new(input.chars(), Loc::default());
        let start = *input.get::<Loc>();
        Self {
            mode: LexMode::Default,
            source,
            input,
            start,
            delim_stack: vec![],
            tokens: vec![],
            spans: vec![],
            minor_errors: vec![],
        }
    }

    fn push_token(mut self, kind: TkKind) -> Self {
        self.tokens.push(kind);
        // TODO Handle balanced delimiters.
        let end = *self.input.get();
        self.spans.push(Span::new(self.start, end));
        self.start = end;
        self
    }

    fn push_err(mut self, reason: Reason) -> Self {
        self.minor_errors.push(LexError {
            loc: *self.input.get::<Loc>(),
            reason,
        });
        self
    }

    fn error_out<T>(self, reason: Reason) -> Result<T> {
        Err((
            LexError {
                loc: *self.input.get::<Loc>(),
                reason,
            },
            LexedBuffer {
                source: self.source,
                kinds: self.tokens.into(),
                spans: self.spans.into(),
                minor_errors: self.minor_errors.into(),
            },
        ))
    }

    fn finalize(self) -> LexedBuffer {
        LexedBuffer {
            source: self.source,
            kinds: self.tokens.into(),
            spans: self.spans.into(),
            minor_errors: self.minor_errors.into(),
        }
    }

    fn trim_whitespaces(&mut self) {
        self.input.skip_while(|c| c.is_whitespace())
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

/// Output of the lexing stage.
#[derive(Debug)]
pub struct LexedBuffer {
    source: IStr,
    kinds: IVec<TkKind>,
    spans: IVec<Span>,
    minor_errors: IVec<LexError>
}

impl LexedBuffer {
    pub fn source(&self) -> IStr {
        self.source.clone()
    }

    pub fn kinds(&self) -> IVec<TkKind> {
        self.kinds.clone()
    }

    pub fn spans(&self) -> IVec<Span> {
        self.spans.clone()
    }

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

    pub fn get_token_txt(&self, id: TkHandle) -> IStr    {
        let span = self.spans[id];
        let range: std::ops::Range<_> = span.into();
        self.source[range].into()
    }

    pub fn print_lexed(&self) {
        for id in 0..self.kinds.len() {
            println!("kind: {} source: {}", self.get_kind(id) , self.get_token_txt(id));
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn match_kinds(found: IVec<TkKind>, expected: Vec<TkKind>) {
        for (found, expected) in found.iter().zip(expected.iter()) {
            assert_eq!(found, expected)
        }
        assert!(found.len() == expected.len());
    }

    #[test]
    fn test_lex_fn_decl() {
        let lexed = lex("comp = f: g: x: f (g x)").unwrap();
        match_kinds(
            lexed.kinds,
            vec![
                Ident, // comp
                Equals,// =
                Ident, // f
                Colon, // :
                Ident, // g
                Colon, // :
                Ident, // x
                Colon, // :
                Ident, // f
                Lpar,  // (
                Ident, // g
                Ident, // x
                Rpar,  // )
            ],
        );
    }

    #[test]
    fn test_rec_dec() {
        let lexed = lex(r#"
            rec $ f: { x @ f, y @ f }
        "#).unwrap();
        match_kinds(
            lexed.kinds,
            vec![
                Ident,  // comp
                Dollar, // $
                Ident,  // f
                Colon,  // :
                Lbrace, // {
                Ident,  // x
                At,     // @
                Ident,  // f
                Comma,  // ,
                Ident,  // y
                At,     // @
                Ident,  // f
                Rbrace, // }
            ],
        );
    }

    #[test]
    fn test_enum_dec() {
        let lexed = lex(r#"
            enum $ a: b: | A a, B b, C |
        "#).unwrap();
        match_kinds(
            lexed.kinds,
            vec![
                Ident,  // comp
                Dollar, // $
                Ident,  // a
                Colon,  // :
                Ident,  // b
                Colon,  // :
                Bar,    // |
                Ident,  // A
                Ident,  // a
                Comma,  // ,
                Ident,  // B
                Ident,  // b
                Comma,  // ,
                Ident,  // C
                Bar,    // |
            ]
        );
    }

    #[test]
    fn test_comment() {
        let lexed = lex(r#"
            -- A comment
            a = 2
            -{ A multiline comment.
               Another line 
            }-
            id = x: x
        "#).unwrap();
        match_kinds(
            lexed.kinds,
            vec![
                Comment,
                Ident,          // a
                Equals,         // =
                Litteral(Int),  // 2
                Comment,        
                Ident,          // id 
                Equals,         // =
                Ident,          // x
                Colon,          // :
                Ident,          // x
            ]
        );
    }

    #[test]
    fn test_literals() {
        let lexed = lex(r#"
            a = 2
            b = 0b10010
            h = 0xff8ea0
            a = -2
            a = -2e-10
            a = 0.5
            s = "test"
        "#).unwrap();
        lexed.print_lexed();
        match_kinds(
            lexed.kinds,
            vec![
                // a = 2
                Ident, Equals, Litteral(Int),
                // b = 0b10010
                Ident, Equals, Litteral(Int), 
                // h = 0xff8ea0
                Ident, Equals, Litteral(Int),  
                // a = -2
                Ident, Equals, Dash, Litteral(Int),
                // a = -2e-10
                Ident, Equals, Dash, Litteral(Float),
                // a = 0.5
                Ident, Equals, Litteral(Float),
                // s = "test"
                Ident, Equals, Litteral(StringLit),
            ]
        );
    }
}
