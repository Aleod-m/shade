use std::fmt::Display;

use crate::lexer;

//pub mod def;
//pub mod expr;
//pub mod ty;

//pub struct Module {
//    defs: Vec<def::Def>,
//}

pub enum ParseError {
    ExpectedSOI,
    ExpectedEOI,
}

pub type NodeHandle = usize;

pub struct ParsedBuffer {
    children: Vec<Vec<NodeHandle>>,
    kinds: Vec<NodeKind>,
    tk: Vec<lexer::TkHandle>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    // Function Declaraiton
    FnDecl,
    FnArg,


    // Expresssions.
    FnApp, // Function Application

    AtomBegin, // `(` Expr `)` 
    AtomEnd,

    IdentValue, // a simple use of a variable.
}

// a: a + 1
// id = x: x
impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::FnApp => write!(f, "Function Application"),
            NodeKind::AtomBegin => write!(f, "Atom Begin"),
            NodeKind::AtomEnd => write!(f, "Atom End"),
            NodeKind::FnDecl => write!(f, "Function Declaration"),
            NodeKind::FnArg => write!(f, "Function Argument"),
            NodeKind::IdentValue => write!(f, "Ident as value"),
        }
    }
}
