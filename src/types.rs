use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        match self {
            Unit => write!(f, "unit"),
            Bool => write!(f, "bool"),
            Int => write!(f, "int"),
            Float => write!(f, "float"),
        }
    }
}

pub enum TypeError {
    OperatorError,
    TypedValueNotPresentInContext,
}

pub trait TypeCheckVisitor {
    fn ty_check(&self, context: &mut HashMap<String, Type>) -> Result<Type, TypeError>;
}
