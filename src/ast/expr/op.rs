use core::fmt;

/// NOT > XOR > AND > OR
/// Mod, Mul, Div > Add, Sub

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Op {
    // Arithmetic Opreators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Function Operators
    Pipe, // "a |> b c" -> b c f
    Dot, // "a.b c"  -> b a c
    // Boolean Opreators
    Eq,
    Neq,
    Not,
    Xor,
    And,
    Or,
}

impl Op {
    pub fn is_bin_op(&self) -> bool {
        use Op::*;
        match self {
            Add | Sub | Mul | Div | Mod => true,
            _ => false,
        }
    }

    pub fn is_bool_op(&self) -> bool {
        use Op::*;
        match self {
            Not | Xor | And | Or => true,
            _ => false,
        }
    }

    pub fn is_unary_op(&self) -> bool {
        use Op::*;
        match self {
            Add | Sub => true,
            _ => false,
        }
    }

    fn prec(&self) -> usize {
        use Op::*;
        match self {
            Add | Sub => 1,
            Mul | Div | Mod => 2,
            Pipe => 0,
            _ => 0,
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Op::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Mod => write!(f, "%"),
            Pipe => write!(f, "|>"),
            Not => write!(f, "!"),
            Xor => write!(f, "^"),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
        }
    }
}
