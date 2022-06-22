use crate::{
    ast::Op,
    lexer::{Token, TokenKind},
    parser::{combinator::and, token, ParseError},
};

mk_parsers! {
    input: &'a [Token];

    simple_op_parser(op: Op) > Op = match input.first() {
        Some(tok) => if Op::from_token_kind(tok.kind) == Some(op) {
            Ok((&input[1..], op))
        } else {
            Err((input, ParseError::UnexpectedToken(tok.clone())))
        },
        None => Err((input, ParseError::UnexpectedEOI)),
    };

    pub op_parser(op: Op) > Op = match op {
        Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod => simple_op_parser(op)(input),
        Op::Pipe => and(
            token(TokenKind::Bar),
            token(TokenKind::GT)
        )(input)
            .map(|(input, _)| (input, Op::Pipe)),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::Op,
        lexer::{Lexer, Token},
    };

    use super::op_parser;

    fn test_op(op: Op) -> bool {
        let input: Vec<Token> = Lexer::new(op.to_string().chars(), None).collect();
        let x = if let Ok((_, pop)) = op_parser(op)(&input) {
             pop == op
        } else {
            false
        }; x
    }

    #[test]
    fn test_op_parser() {
        assert!(test_op(Op::Add));
        assert!(test_op(Op::Sub));
        assert!(test_op(Op::Mul));
        assert!(test_op(Op::Pipe));
        assert!(test_op(Op::Mod));
        assert!(test_op(Op::Div));
    }
}
