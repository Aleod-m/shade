use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    Vec{ty:Box<Type>, dimension: usize},
    List{ty:Box<Type>},
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type::*;
        match self {
            Unit => write!(f, "unit"),
            Bool => write!(f, "bool"),
            Int => write!(f, "int"),
            Float => write!(f, "float"),
            Vec { dimension, .. } => write!(f, "vec{}", dimension),
            List { ty } => write!(f, "[{}]", ty),
        }
    }
}
