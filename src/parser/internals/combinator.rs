use std::marker::PhantomData;

use crate::lexer::Token;

use super::{ParseError, Parser, ParserRes};

pub struct Map<P, G, O>
where
    P: Parser,
    G: Fn(P::Output) -> O,
{
    pub p: P,
    pub g: G,
    pub _m: core::marker::PhantomData<G::Output>,
}

impl<P, G, O> Parser for Map<P, G, O>
where
    P: Parser,
    G: Fn(P::Output) -> O,
{
    type Output = G::Output;
    fn parse<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output> {
        let parse = self.p.parse_or_reset(input);
        match parse {
            Ok((input, res)) => Ok((input, (self.g)(res))),
            Err(e) => Err(e),
        }
    }
}

pub struct Or<P1, P2, O>
where
    P1: Parser<Output = O>,
    P2: Parser<Output = O>,
{
    pub f: P1,
    pub s: P2,
    pub _m : PhantomData<P1::Output>,
}

impl<P1, P2, O> Parser for Or<P1, P2, O>
where
    P1: Parser<Output = O>,
    P2: Parser<Output = O>,
{
    type Output = O;
    fn parse<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output> {
        self.f
            .parse_or_reset(input)
            .or_else(|(input, _)| self.s.parse_or_reset(input))
    }
}

pub struct And<P1, P2>
where
    P1: Parser,
    P2: Parser,
{
    pub f: P1,
    pub s: P2,
    pub _m1: core::marker::PhantomData<P1::Output>,
    pub _m2: core::marker::PhantomData<P2::Output>,
}

impl<P1, P2> Parser for And<P1, P2>
where
    P1: Parser,
    P2: Parser,
{
    type Output = (P1::Output, P2::Output);
    fn parse<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
        let (input, result1) = self.f.parse_or_reset(input)?;
        self.s
            .parse_or_reset(input)
            .map(|(input, result2)| Ok((input, (result1, result2))))?
    }
}

pub struct Not<P>
where
    P: Parser,
{
    pub f: P,
    pub _m: core::marker::PhantomData<P::Output>,
}

impl<P> Parser for Not<P>
where
    P: Parser,
{
    type Output = ();
    fn parse<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
        let parse = self.f.parse_and_reset(input);
        match parse {
            Ok((input, _)) => {
                return Err((input, ParseError::ParserFailed));
            }
            Err((input, _)) => return Ok((input, ())),
        }
    }
}

pub struct Peek<P>
where
    P: Parser,
{
    pub f: P,
    pub _m: core::marker::PhantomData<P::Output>,
}

impl<P> Parser for Peek<P>
where
    P: Parser,
{
    type Output = P::Output;
    fn parse<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
        self.f.parse_and_reset(input)
    }
}

pub struct OneOrMore<P>
where
    P: Parser,
{
    pub p: P,
    pub _m: core::marker::PhantomData<P::Output>,
}

impl<P> Parser for OneOrMore<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;

    fn parse<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
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

pub struct ZeroOrMore<P>
where
    P: Parser,
{
    pub p: P,
    pub _m: core::marker::PhantomData<P::Output>,
}

impl<P> Parser for ZeroOrMore<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;
    fn parse<'a>(&self, input: &'a[Token]) -> ParserRes<'a, Self::Output> {
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
