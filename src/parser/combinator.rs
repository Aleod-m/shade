use crate::lexer::Lexer;

use super::{ParseError, Parser};

pub struct Map<P, G, O1, O2>
where
    P: Parser<O1>,
    G: Fn(O1) -> O2,
{
    pub p: P,
    pub g: G,
    pub _m1: core::marker::PhantomData<O1>,
    pub _m2: core::marker::PhantomData<O2>,
}

impl<P, G, O1, O2> Parser<O2> for Map<P, G, O1, O2>
where
    P: Parser<O1>,
    G: Fn(O1) -> O2,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<O2> {
        let parse = self.p.parse_or_reset(input);
        match parse {
            Ok((input, res)) => Ok((input, (self.g)(res))),
            Err(e) => Err(e),
        }
    }
}

pub struct Or<P, O>
where
    P: Parser<O>,
{
    pub f: P,
    pub s: P,
    pub _m: core::marker::PhantomData<O>,
}

impl<P, O> Parser<O> for Or<P, O>
where
    P: Parser<O>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<O> {
        self.f
            .parse_or_reset(input)
            .or_else(|(input, _)| self.s.parse_or_reset(input))
    }
}

pub struct And<P1, P2, O1, O2>
where
    P1: Parser<O1>,
    P2: Parser<O2>,
{
    pub f: P1,
    pub s: P2,
    pub _m1: core::marker::PhantomData<O1>,
    pub _m2: core::marker::PhantomData<O2>,
}

impl<P1, P2, O1, O2> Parser<(O1, O2)> for And<P1, P2, O1, O2>
where
    P1: Parser<O1>,
    P2: Parser<O2>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<(O1, O2)> {
        let (input, result1) = self.f.parse_or_reset(input)?;
        self.s
            .parse_or_reset(input)
            .map(|(input, result2)| Ok((input, (result1, result2))))?
    }
}

pub struct Not<P, O>
where
    P: Parser<O>,
{
    pub f: P,
    pub _m: core::marker::PhantomData<O>,
}

impl<P, O> Parser<()> for Not<P, O>
where
    P: Parser<O>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<()> {
        let parse = self.f.parse_and_reset(input);
        match parse {
            Ok((input, _)) => {
                let loc = input.loc();
                return Err((input, ParseError::UnexpectedToken(loc)));
            }
            Err((input, _)) => return Ok((input, ())),
        }
    }
}

pub struct Peek<P, O>
where
    P: Parser<O>,
{
    pub f: P,
    pub _m: core::marker::PhantomData<O>,
}

impl<P, O> Parser<O> for Peek<P, O>
where
    P: Parser<O>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<O> {
        self.f.parse_and_reset(input)
    }
}

pub struct OneOrMore<P, O>
where
    P: Parser<O>,
{
    pub p: P,
    pub _m: core::marker::PhantomData<O>,
}

impl<P, O> Parser<Vec<O>> for OneOrMore<P, O>
where
    P: Parser<O>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<Vec<O>> {
        let mut input = input;
        let mut result = vec![];
        match self.p.parse(input) {
            Ok((lex, res)) => {
                input = lex;
                result.push(res);
            }
            Err(e) => return Err(e),
        }
        loop {
            match self.p.parse(input) {
                Ok((lex, res)) => {
                    input = lex;
                    result.push(res);
                }
                Err((lex, _)) => {
                    input = lex;
                    break;
                }
            }
        }
        return Ok((input, result));
    }
}

pub struct ZeroOrMore<P, O>
where
    P: Parser<O>,
{
    pub p: P,
    pub _m: core::marker::PhantomData<O>,
}

impl<P, O> Parser<Vec<O>> for ZeroOrMore<P, O>
where
    P: Parser<O>,
{
    fn parse(&mut self, input: Lexer) -> super::ParserRes<Vec<O>> {
        let mut input = input;
        let mut result = vec![];
        loop {
            match self.p.parse(input) {
                Ok((lex, res)) => {
                    input = lex;
                    result.push(res);
                }
                Err((lex, _)) => {
                    input = lex;
                    break;
                }
            }
        }
        return Ok((input, result));
    }
}
