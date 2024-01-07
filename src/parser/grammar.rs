
use crate::lexer::token::TkKind;
use super::parsetree::NodeKind;

use super::{ParserRes::*, Parser};
use NodeKind::*;
use TkKind::*;

mk_parsers! {
    input = parsed;

    /// The most basic parser that parses a single token:
    /// - Succed if the next token in the input is of `kind`.
    /// - Fails otherwise.
    pub token(kind : TkKind) = {
        if parsed.is_kind(kind) {
            Succ
        } else {
            Fail
        }
    };

    pub basic(tk: TkKind, node: NodeKind) = {
        just { token(tk) => { push node } } 
    };


    pub assign() = {
        seq {
            token(Ident) => { stack LAssign },
            token(Equals) => { push Assign },
            expr(),
        } 
        then {
            pop
        }
    };

    pub expr() = {
        choice {
            atom(expr()),
            fn_app(),
            function(),
            basic(TkKind::Ident, NodeKind::IdentValue),
        }
    };


    pub atom(p: impl Parser) = {
        seq {
            token(TkKind::Lpar) => { stack NodeKind::AtomBegin },
            p,
            token(TkKind::Rpar) => { stack NodeKind::AtomEnd },
        }
        then { pop; pop }
    };

    pub fn_app() = {
        seq {
            fn_app_left() => { stack NodeKind::FnApp },
            expr(),
        }
        then { pop }
    };

    pub fn_app_left() = { 
        choice {
            atom(function()),
            basic(TkKind::Ident, NodeKind::IdentValue),
        }
    };


    pub function() = {
        seq {
            token(TkKind::Ident) => { push NodeKind::FnArg },
            token(TkKind::Colon) => { stack NodeKind::FnDecl },
            expr(),
        }
        then { pop }
    };
}
