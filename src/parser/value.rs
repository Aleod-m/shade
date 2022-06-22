use crate::{
    ast::Value,
    lexer::{Token, TokenKind},
};

use super::{
    token,
    combinator::{and, map, or},
};

mk_parsers! {
    input: &'a [Token];
    /// Parses the unit value:
    /// - ()
    pub unit_parser() > Value = {
        map(
            and(
                token(TokenKind::Lpar),
                token(TokenKind::Rpar)),
            |_| { Value::new(()) }
        )(input)
    };

    /// Parses integer values :
    /// - Hexadecimal: 0xffa3b2
    /// - Binary : 0b101001
    /// - Decimal : 42
    /// This parser can panic. This means there was an ill formed
    /// int token.
    pub int_parser() > Value = {
        map(token(TokenKind::Int), |tok| {
            let text = tok.text;
            let simple = text.parse::<i32>();
            if let Ok(val) = simple {
                return Value::new(val);
            }
            if text.starts_with("0x") {
                let text = text.trim_start_matches("0x");
                return Value::new(i32::from_str_radix(text, 16).unwrap());
            } else if text.starts_with("0b") {
                let text = text.trim_start_matches("0b");
                return Value::new(i32::from_str_radix(text, 2).unwrap());
            }
            panic!("There was an incorrect tokenized integer at {}!", tok.loc);
        })(input)
    };


    /// Parses floating point values:
    /// - classic : 0.5
    /// - with exponent : 5.2e-6
    /// This parser can panic. This means there was an ill formed
    /// float token.
    pub float_parser() > Value = {
        map(token(TokenKind::Float), |tok| {
            Value::new(tok.text.parse::<f32>().unwrap())
        })(input)
    };

    /// Parses either a float or an int
    pub number_parser() > Value = {
        or(int_parser(), float_parser())(input)
    };

    /// Parses a value:
    ///
    /// Here is the rule:
    /// value <- unit | int | float
    pub value_parser() > Value = {
        or(or(unit_parser(), int_parser()), float_parser())(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::Value,
        lexer::{Lexer, Token},
        parser::value_parser,
    };

    #[test]
    fn test_value_parser_int() {
        let input_int: Vec<Token> = Lexer::new("12".chars(), None).into_iter().collect();
        let result_int = value_parser()(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(12), value_int);

        let input_int: Vec<Token> = Lexer::new("0b1010".chars(), None).into_iter().collect();
        let result_int = value_parser()(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(10), value_int);

        let input_int: Vec<Token> = Lexer::new("0xff".chars(), None).into_iter().collect();
        let result_int = value_parser()(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(255), value_int);
    }

    #[test]
    fn test_value_parser_float() {
        let input_float: Vec<Token> = Lexer::new("3e-10".chars(), None).into_iter().collect();
        let result_float = value_parser()(&input_float);
        let (_, value_float) = result_float.unwrap();
        assert_eq!(Value::new(3e-10), value_float);
        let input_float: Vec<Token> = Lexer::new("3.5".chars(), None).into_iter().collect();
        let result_float = value_parser()(&input_float);
        let (_, value_float) = result_float.unwrap();
        assert_eq!(Value::new(3.5), value_float);
    }

    #[test]
    fn test_value_parser_unit() {
        let input_unit: Vec<Token> = Lexer::new("()".chars(), None).into_iter().collect();
        let result_unit = value_parser()(&input_unit);
        let (_, value_unit) = result_unit.unwrap();
        assert_eq!(Value::new(()), value_unit);
    }

    //    #[test]
    //    fn test_value_parser_list() {
    //        let parser = ValueParser;
    //
    //        let input_unit: Vec<Token> = Lexer::new("[1 12 0xFF 0b1010]".chars(), None).into_iter().collect();
    //        let result_unit= parser.parse(&input_unit);
    //        let (_, value_unit) = result_unit.unwrap();
    //        assert_eq!(Value::new(vec![Value::new(1), Value::new(12), Value::new(255), Value::new(10)]), value_unit);
    //    }
}
