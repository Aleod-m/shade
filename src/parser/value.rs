use crate::{ast::Value, lexer::{TokenKind, Token}};

use super::{Parser, TokenParser, ParserRes};

fn unit() -> impl Parser<Output = Value> {
    TokenParser(TokenKind::Lpar)
        .and(TokenParser(TokenKind::Rpar))
        .map(|_| Value::new(()))
}

fn int() -> impl Parser<Output = Value> {
    TokenParser(TokenKind::Int).map(|tok| {
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
        panic!("There was an incorrect tokenized integer!");
    })
}

fn list() -> impl Parser<Output = Value> {
    todo!();
    #[allow(unreachable_code)]
    TokenParser(TokenKind::LBracket)
        .and(
            ValueParser.and(TokenParser(TokenKind::Comma)).zero_or_more()
        )
        .and(TokenParser(TokenKind::RBracket))
        .map(|o| Value::new(o.0.1.map(|v| v.0)))
}


fn float() -> impl Parser<Output = Value> {
    TokenParser(TokenKind::Float).map(|tok| Value::new(tok.text.parse::<f32>().unwrap()))
}

struct ValueParser;
impl Parser for ValueParser {
    type Output = Value;
    fn parse<'a>(&self, input: &'a [Token]) -> ParserRes<'a, Self::Output> {
        unit().or(int()).or(float()).or(list()).parse(input)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::Value,
        lexer::{Lexer, Token},
    };

    use super::{Parser, ValueParser};

    #[test]
    fn test_value_parser_int() {
        let parser = ValueParser;

        let input_int: Vec<Token> = Lexer::new("12".chars(), None).into_iter().collect();
        let result_int = parser.parse(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(12), value_int);
        
        let input_int: Vec<Token> = Lexer::new("0b1010".chars(), None).into_iter().collect();
        let result_int = parser.parse(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(10), value_int);

        let input_int: Vec<Token> = Lexer::new("0xff".chars(), None).into_iter().collect();
        let result_int = parser.parse(&input_int);
        let (_, value_int) = result_int.unwrap();
        assert_eq!(Value::new(255), value_int);
    }

    #[test]
    fn test_value_parser_float() {
        let parser = ValueParser;

        let input_float: Vec<Token> = Lexer::new("3e-10".chars(), None).into_iter().collect();
        let result_float = parser.parse(&input_float);
        let (_, value_float) = result_float.unwrap();
        assert_eq!(Value::new(3e-10), value_float);
        let input_float: Vec<Token> = Lexer::new("3.5".chars(), None).into_iter().collect();
        let result_float = parser.parse(&input_float);
        let (_, value_float) = result_float.unwrap();
        assert_eq!(Value::new(3.5), value_float);
    }

    #[test]
    fn test_value_parser_unit() {
        let parser = ValueParser;
         
        let input_unit: Vec<Token> = Lexer::new("()".chars(), None).into_iter().collect();
        let result_unit= parser.parse(&input_unit);
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
