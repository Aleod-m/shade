use super::{Parser, ParseError};
use crate::lexer::Token;

pub fn map<'a, P, F, O1, O2>(parser: P, f: F) -> impl Parser<'a, O2>
where
    P: Parser<'a,  O1>,
    F: Fn(O1) -> O2,
{
    move |input: &'a [Token]| parser(input).map(|(input, val)| (input, f(val)))
}

pub fn not<'a, P, O>(parser: P) -> impl Parser<'a, ()>
where
    P: Parser<'a,  O>,
{
    move |input: &'a [Token]| match parser(input) {
        Ok(_) => Err((input, ParseError::ParserFailed)),
        Err((input, _)) => Ok((input, ())),
    }
}

pub fn opt<'a, P, O>(parser: P) -> impl Parser<'a, Option<O>>
where
    P: Parser<'a,  O>,
{
    move |input: &'a [Token]| match parser(input) {
        Ok((input, val)) => Ok((input, Some(val))),
        Err(_) => Ok((input, None)),
    }
}

pub fn many0<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
where
    P: Parser<'a,  O>,
{
    move |input: &'a [Token]| {
        let mut vals = Vec::new();
        let mut i = input;
        while let Ok((ri, v)) = parser(i) {
            i = ri;
            vals.push(v);
        }
        Ok((i, vals))
    }
}

pub fn many1<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
where
    P: Parser<'a,  O>,
{
    move |input: &'a [Token]| {
        let mut vals = Vec::new();
        let mut i = input;
        if let Ok((ri, v)) = parser(i) {
            i = ri;
            vals.push(v);
            while let Ok((ri, v)) = parser(i) {
                i = ri;
                vals.push(v);
            }
            Ok((i, vals))
        } else {
            Err((input, ParseError::ParserFailed))
        }
    }
}

pub fn and<'a, P1, P2, O1, O2>(p1: P1, p2: P2) -> impl Parser<'a, (O1, O2)>
where
    P1: Parser<'a,  O1>,
    P2: Parser<'a,  O2>,
{
    move |input: &'a [Token]| {
        let (input, result1) = p1(input)?;
        p2(input).map(|(input, result2)| Ok((input, (result1, result2))))?
    }
}


pub fn or<'a, P1, P2, O>(p1: P1, p2: P2) -> impl Parser<'a, O>
where
    P1: Parser<'a,  O>,
    P2: Parser<'a,  O>,
{
    move |input: &'a [Token]| {
        p1(input)
            .or_else(|(input, _)| p2(input))
    }
}

pub fn between<'a, LP, P, RP, LO, O, RO>(p: P, lp: LP, rp: RP) -> impl Parser<'a, (LO, O, RO)>
where
    LP: Parser<'a,  LO>,
    P: Parser<'a,  O>,
    RP: Parser<'a,  RO>
{
    move |input: &'a [Token]| {
        let (ri, lval) = lp(input)?;
        let (ri, val) = p(ri)?;
        let (ri, rval) = rp(ri)?;
        Ok((ri, (lval, val, rval)))
    }
}

//pub fn ordered_choice<'a, P, O, I>(ps: I) -> impl Parser<'a, O> 
//where
//    P: Parser<'a,  O>,
//    I: IntoIterator<Item = P>,
//{
//    move |input: &'a [Token]| {
//        let res;
//        for p in ps.into_iter() {
//            res = p(input);
//            if res.is_ok() {
//               return res; 
//            }
//        }
//        return res;
//    }
//
//}
