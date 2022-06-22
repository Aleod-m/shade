use crate::ast::Stmt;
use crate::lexer::{Token, TokenKind};

pub mod combinator;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOI,
    ParserFailed,
    ValueParsedErroned,
}

pub struct ParserState {
    context: Vec<Stmt>,
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            context: Vec::new(),
        }
    }
}

pub type ParserRes<'a, O> = Result<(&'a [Token], O), (&'a [Token], ParseError)>;

pub trait Parser<'a, O> : Fn(&'a[Token]) -> ParserRes<'a, O> {}
impl<'a, T, O> Parser<'a, O> for T
where
    T: Fn(&'a[Token]) -> ParserRes<'a, O> {}

/// Here is a macro for automating parser creation.
macro_rules! mk_parsers {
    (
    $input:ident : $inty:ty;
    $($(#[doc= $doc:expr])*$v:vis $name:ident($($param:ident : $pty:ty),*) > $output:ty = $rule:expr);+
    ) => {
        $(
            $(#[doc = $doc])*
            $v fn $name<'a>($($param:$pty),*) -> impl crate::parser::internals::Parser<'a, $output> {
                move |$input: $inty| {
                    $rule
                }
            }
        )+
    };
}

mk_parsers!{
    input: &'a [Token];

    /// The most basic parser that parses a single token:
    /// - Succed if the next token in the input is of `kind`.
    /// - Fails otherwise.
    pub token(kind : TokenKind) > Token = match input.first() {
        Some(tok) if tok.kind == kind => Ok((&input[1..], tok.clone())),
        Some(tok) => Err((input, ParseError::UnexpectedToken(tok.clone()))),
        None => Err((input, ParseError::UnexpectedEOI)),
    }
}
