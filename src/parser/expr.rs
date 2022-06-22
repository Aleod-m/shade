use crate::{ast::{Expr, Op}, lexer::{TokenKind, Token}};

use super::{combinator::{or, many0, and, map, between}, token, number_parser};



mk_parsers!{
    input: &'a [Token];

    /// Term <- Number | '(' Expr ')'
    term_parser() > Expr = {
        or(
            map(number_parser(), Expr::Value),
            map(
                between(
                    expr_parser(),
                    token(TokenKind::Lpar),
                    token(TokenKind::Rpar),
                ),
                |(_, expr, _)| expr,
            ),
        )(input)
    };

    /// UnaryExpr <- ('+' | '-') Term
    pub unary_parser() > Expr = {
            map(
                and(
                    or(token(TokenKind::Plus), token(TokenKind::Dash)),
                    term_parser(),
                ),
                |(token, term)| Expr::unary(Op::from_token_kind(token.kind).unwrap(), term),
            )(input)
        
    };

    /// Product <- Term (('*' | '/') Term)*
     product_parser() > Expr = {
            map(
                and(
                    term_parser(),
                    many0(and(
                        or(token(TokenKind::Star), token(TokenKind::Slash)),
                        term_parser(),
                    )),
                ),
                |(expr, others): (Expr, Vec<(Token, Expr)>)| {
                    others.into_iter().fold(expr, |lhs, (op_tok, rhs)| {
                        Expr::binary(Op::from_token_kind(op_tok.kind).unwrap(), lhs, rhs)
                    })
                },
            )(input)
        };
    

    /// BinExpr <- Product (('+' | '-') Product)*
     binary_parser() > Expr = {
            map(
                and(
                    product_parser(),
                    many0(and(
                        or(token(TokenKind::Plus), token(TokenKind::Dash)),
                        product_parser(),
                    )),
                ),
                |(expr, others): (Expr, Vec<(Token, Expr)>)| {
                    others.into_iter().fold(expr, |lhs, (op_tok, rhs)| {
                        Expr::binary(Op::from_token_kind(op_tok.kind).unwrap(), lhs, rhs)
                    })
                },
            )(input)
        };
    

    /// Expr <- BinExpr | UnaryExpr | Ident | Value
    pub expr_parser() > Expr = {
        or(binary_parser(), unary_parser())(input)
    }
}


#[cfg(test)]
mod test {
    use crate::{lexer::{Lexer, Token}, parser::{ParseError, expr::binary_parser}, ast::{Expr, Op}};

    use super::unary_parser;

    #[test]
    fn test_parse_unary() -> Result<(), ParseError> {
        let input: Vec<Token> = Lexer::new("-1".chars(), None).collect();
        let parse_res: Expr = unary_parser()(&input).unwrap().1;
        assert_eq!(parse_res, Expr::unary(Op::Sub, Expr::value(1)));
        Ok(())
    }

    #[test]
    fn test_parse_binary() -> Result<(), ParseError> {
        let input: Vec<Token> = Lexer::new("3 * 5 + 2".chars(), None).collect();
        let parse_res: Expr = binary_parser()(&input).unwrap().1;
        let expected: Expr = Expr::binary(
            Op::Add, 
            Expr::binary(
                Op::Mul,
                Expr::value(3),
                Expr::value(5),
            ), 
            Expr::value(2),
        );
        assert_eq!(parse_res, expected);
        Ok(())
    }


    #[test]
    fn test_parse_binary_with_term() -> Result<(), ParseError> {
        let input: Vec<Token> = Lexer::new("3 * (5 + 2)".chars(), None).collect();
        let parse_res: Expr = binary_parser()(&input).unwrap().1;
        let expected: Expr = Expr::binary(
            Op::Mul,
            Expr::value(3),
            Expr::binary(
                Op::Add, 
                Expr::value(5),
                Expr::value(2),
            ), 
        );
        assert_eq!(parse_res, expected);
        Ok(())
    }

    #[test]
    fn test_parse_binary_with_term_and_floats() -> Result<(), ParseError> {
        let input: Vec<Token> = Lexer::new("3. * (5e-1 + 2.)".chars(), None).collect();
        let parse_res: Expr = binary_parser()(&input).unwrap().1;
        let expected: Expr = Expr::binary(
            Op::Mul,
            Expr::value(3.),
            Expr::binary(
                Op::Add, 
                Expr::value(5e-1),
                Expr::value(2.),
            ), 
        );
        assert_eq!(parse_res, expected);
        Ok(())
    }
}
