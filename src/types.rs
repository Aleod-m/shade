use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    List(Box<Type>),
    Undefined,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        match self {
            Unit => write!(f, "unit"),
            Bool => write!(f, "bool"),
            Int => write!(f, "int"),
            Float => write!(f, "float"),
            List(ty) => write!(f, "list {}", ty), 
            Undefined => write!(f, "undefined type"), 
        }
    }
}

pub enum TypeError {
    OperatorError,
    TypedValueNotPresentInContext,
    NonUniformTypeInList,
}

pub type TypeRes = Result<Type, TypeError>;

pub trait TypeCheckVisitor {
    fn ty_check(&self, context: &mut HashMap<String, Type>) -> Result<Type, TypeError>;
}
