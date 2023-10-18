#[macro_use]
mod internals;

pub mod parsetree;
pub use internals::*;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("NONE ERROR.")]
    NONE,
}

use parsetree::NodeKind;
use crate::lexer::token::TkKind;

use ParserRes::*;

mk_parsers! {
    input = parsed;
    /// The most basic parser that parses a single token:
    /// - Succed if the next token in the input is of `kind`.
    /// - Fails otherwise.
    pub token(kind : TkKind) = (expr : if parsed.is_kind(kind) {
            Succ
        } else {
            Fail
        }
    );

    pub basic(tk: TkKind, node: NodeKind) = (seq:
        token(tk) => { push node }
    );

    /// A parser that tries to parse an specific keyword.
    //pub keyword(kw: KeyWord) = ( expr: Fail );

    pub expr() = (choice:
        ; atom(expr())
        ; fn_app()
        ; function()
        ; basic(TkKind::Ident, NodeKind::IdentValue)
    );


    pub atom(p: impl Parser) = (seq:
        token(TkKind::Lpar) => { stack NodeKind::AtomBegin }
        p => {}
        token(TkKind::Rpar) => { stack NodeKind::AtomEnd }
        ; after { pop; pop }
    );

    pub fn_app() = (seq:
        fn_app_left() => { stack NodeKind::FnApp }
        expr() => { }
        ; after { pop }
    );

    pub fn_app_left() = (choice:
        ; atom(function())
        ; basic(TkKind::Ident, NodeKind::IdentValue)
    );


    pub function() = (seq:
        token(TkKind::Ident) => { push NodeKind::FnArg }
        token(TkKind::Colon) => { stack NodeKind::FnDecl }
        expr() => { }
        ; after  { pop }
    )
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, utils::IStr};

    use super::{state::*, *};
    use NodeKind::*;

    fn verify_nodes<'a>(pb: ParsedBuffer, expected: Vec<NodeKind>) {
        for (&e_kind, &f_kind) in expected.iter().zip(pb.nodes().iter()) {
            assert_eq!(e_kind, f_kind, "Expected {}, Found {}", e_kind, f_kind);
        }
    }

    fn new_state<'a>(input: &str) -> ParserState {
        let input: IStr = input.into();
        ParserState::new(Lexer::new(&input.clone()).lex().unwrap())
    }

    #[test]
    fn test_basic_parser() {
        let pb = new_state("(f: f) a").run_parser(expr());
        let expected = vec![

            IdentValue, // f
            FnArg, // f
            FnDecl, // :
            AtomEnd, //)
            AtomBegin, //(
            
            IdentValue, // a

            FnApp, // ?
        ];
        verify_nodes(pb, expected)
    }

    #[test]
    fn test_2_fn_app_parser() {
        let pb = new_state("(f: g: f g) (x: x) a").run_parser(expr());
        let expected = vec![
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
        ];
        verify_nodes(pb, expected)
    }
}
