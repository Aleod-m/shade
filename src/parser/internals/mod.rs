use crate::ast::Stmt;
use crate::lexer::{TokenKind, Token};

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

pub type ParserRes<'a, O> = Result<(&'a[Token], O), (&'a [Token], ParseError)> ;

pub trait Parser {
    type Output;
    fn parse<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output>;

    fn parse_or_reset<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output> {
        self.parse(input).or_else(|(_, error)| {
            Err((input, error))
        })

    }

    fn parse_and_reset<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
        match self.parse(input) {
            Ok((lex, res)) => {
                Ok((lex, res))  
            }
            Err((_, err)) => {
                Err((input, err))  
            }
        }

    }

    fn map<G, O>(self, g: G) -> combinator::Map<Self, G, O>
    where
        G: Fn(Self::Output) -> O,
        Self: Sized,
    {
        combinator::Map {
            p: self,
            g,
            _m: core::marker::PhantomData,
        }
    }

    fn or<P>(self, s: P) -> combinator::Or<Self, P, Self::Output>
    where
        Self: Sized,
        P: Parser<Output = Self::Output>,
    {
        combinator::Or {
            f: self,
            s,
            _m: core::marker::PhantomData,
        }
    }

    fn and<P, O>(self, s: P) -> combinator::And<Self, P>
    where
        P: Parser<Output = O>,
        Self: Sized,
    {
        combinator::And {
            f: self,
            s,
            _m1: core::marker::PhantomData,
            _m2: core::marker::PhantomData,
        }
    }

    fn not(self) -> combinator::Not<Self>
    where
        Self: Sized,
    {
        combinator::Not {
            f: self,
            _m : core::marker::PhantomData,
        }
    }

    fn peek(self) -> combinator::Peek<Self>
    where
        Self: Sized,
    {
        combinator::Peek {
            f: self,
            _m: core::marker::PhantomData
        }
    }

    fn one_or_more(self) -> combinator::OneOrMore<Self>
    where
        Self: Sized,
    {
       combinator::OneOrMore {
            p: self,
            _m: core::marker::PhantomData,
        }
    }

    fn zero_or_more(self) -> combinator::ZeroOrMore<Self>
    where
        Self: Sized,
    {
        combinator::ZeroOrMore {
            p: self,
            _m: core::marker::PhantomData,
        }
    }
}

pub struct TokenParser(pub TokenKind);
impl Parser for TokenParser {
    type Output = Token;
    fn parse<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output> {
        match input.first() {
            Some(tok) if tok.kind == self.0 => {
                Ok((&input[1..], tok.clone()))
            },
            Some(tok) => {
                Err((input, ParseError::UnexpectedToken(tok.clone())))
            }
            None => {
                Err((input, ParseError::UnexpectedEOI))
            }
        }
    }
}

