use super::Ident;

#[derive(Debug)]
pub enum TypeExpr {
    TypeFn { arg: Ident, expr: Box<TypeExpr> },
    Record { fields: Vec<(Ident, TypeExpr)> },
    Enum { variants: Vec<(Ident, TypeExpr)> },
    Array(Box<TypeExpr>),
    Primitive(PrimitiveType),
    NamedType(Ident),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
    Float,
    String,
    Unit,
}
