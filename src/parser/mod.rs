use crate::ast::Stmt;
use crate::lexer::{TokenKind, Lexer, Token, Loc};

mod combinator;

pub enum ParseError {
    UnexpectedToken(Loc),
    ParserFailed,
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

pub type ParserRes<O> = Result<(Lexer, O), (Lexer, ParseError)> ;

pub trait Parser<O> {
    fn parse(&mut self, input: Lexer) -> ParserRes<O>;

    fn parse_or_reset(&mut self, input: Lexer) -> ParserRes<O> {
        let pos = input.pos();
        self.parse(input).or_else(|(mut input, error)| {
            input.reset_pos(pos);
            Err((input, error))
        })

    }

    fn parse_and_reset(&mut self, input: Lexer) -> ParserRes<O> {
        let pos = input.pos();
        match self.parse(input) {
            Ok((lex, res)) => {
                Ok((lex, res))  
            }
            Err((mut lex, err)) => {
                lex.reset_pos(pos);
                Err((lex, err))  
            }
        }

    }

    fn map<G, O2>(self, g: G) -> combinator::Map<Self, G, O, O2>
    where
        G: Fn(O) -> O2,
        Self: Sized,
    {
        combinator::Map {
            p: self,
            g,
            _m1: core::marker::PhantomData,
            _m2: core::marker::PhantomData,
        }
    }

    fn or(self, s: Self) -> combinator::Or<Self, O>
    where
        Self: Sized,
    {
        combinator::Or {
            f: self,
            s,
            _m: core::marker::PhantomData,
        }
    }

    fn and<P2, O2>(self, s: P2) -> combinator::And<Self, P2, O, O2>
    where
        P2: Parser<O2>,
        Self: Sized,
    {
        combinator::And {
            f: self,
            s,
            _m1: core::marker::PhantomData,
            _m2: core::marker::PhantomData,
        }
    }

    fn not(self) -> combinator::Not<Self, O>
    where
        Self: Sized,
    {
        combinator::Not {
            f: self,
            _m : core::marker::PhantomData,
        }
    }

    fn peek(self) -> combinator::Peek<Self, O>
    where
        Self: Sized,
    {
        combinator::Peek {
            f: self,
            _m: core::marker::PhantomData
        }
    }

    fn one_or_more(self) -> combinator::OneOrMore<Self, O>
    where
        Self: Sized,
    {
       combinator::OneOrMore {
            p: self,
            _m: core::marker::PhantomData,
        }
    }

    fn zero_or_more(self) -> combinator::ZeroOrMore<Self, O>
    where
        Self: Sized,
    {
        combinator::ZeroOrMore {
            p: self,
            _m: core::marker::PhantomData,
        }
    }
}

pub struct TokenParser(TokenKind);
impl Parser<Token> for TokenParser {
    fn parse(&mut self, mut input: Lexer) -> ParserRes<Token> {
        let tok = input.next_tok();
        if tok.kind == self.0 {
            Ok((input, tok))
        } else {
            let loc = input.loc();
            Err((input, ParseError::UnexpectedToken(loc)))
        }
    }
}

#[allow(unused_macros)]
macro_rules! token {
    ($ty: ident) => {
        TokenParser(ident)
    };
}
