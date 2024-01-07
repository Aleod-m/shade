use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    // Function Declaraiton
    FnDecl,
    FnArg,

    // Assign
    Assign,
    LAssign,

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
            NodeKind::Assign => write!(f, "Assign statement"),
            NodeKind::LAssign => write!(f, "Left side of the assignement"),
        }
    }
}
