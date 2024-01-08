#[macro_use]
mod internals;

pub mod parsetree;
pub use internals::*;
use thiserror::Error;
mod grammar;



#[derive(Error, Debug)]
pub enum ParseError {
    #[error("NONE ERROR.")]
    NONE,
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, utils::IStr};

    use super::{state::*, grammar::*, *};
    use parsetree::NodeKind::{self, *};

    fn verify_nodes<'a>(pb: ParsedBuffer, expected: Vec<NodeKind>) {
        for (&e_kind, &f_kind) in expected.iter().zip(pb.nodes().iter()) {
            assert_eq!(e_kind, f_kind, "Expected {}, Found {}", e_kind, f_kind);
        }
    }

    fn new_state<'a>(input: &str) -> ParserState {
        let input: IStr = input.into();
        ParserState::new(Lexer::new(&input.clone()).lex().unwrap())
    }

    macro_rules! mk_test {
        ($name:ident, $input:literal, $parser:expr, $expected:tt) => {
            #[test]
            fn $name() {
                let pb = new_state($input).run_parser($parser);
                verify_nodes(pb, vec!$expected)
            }
        };
    }

    mk_test!(test_assign, "a = x: x", assign(), [
        Assign, // = 
        LAssign, // a
        FnDecl, // :
        FnArg, // x
        IdentValue, // x
    ]);

    mk_test!(test_fn_app, "(f: f) a", expr(), [
        IdentValue, // f
        FnArg, // f
        FnDecl, // :
        AtomEnd, //)
        AtomBegin, //(
        
        IdentValue, // a

        FnApp, // ?
    ]);

    mk_test!(test_fn_app_2, "(f: g: f g) (x: x) a", expr(), [
        IdentValue, // g
        IdentValue, // f
        FnApp, // ?
        
        FnArg, // g
        FnDecl, //:
        FnArg, // f
        FnDecl, // :
        AtomEnd, //)
        AtomBegin, //(

        IdentValue, // x
        FnArg, // x
        FnDecl, // :
        AtomEnd, //)
        AtomBegin, //(

        FnApp, // ?

        IdentValue, // a
        
        FnApp, // ?
    ]);

}
