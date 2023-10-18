use super::{expr::Expr, ty::TypeExpr, Ident};

#[derive(Debug)]
pub enum Def {
    Fn {
        ident: Ident,
        ty: Option<TypeExpr>,
        expr: Expr,
    },
    Type {
        ident: Ident,
        ty: TypeExpr,
    },
    Binding {
        ident: Ident,
        ty: TypeExpr,
    },
}

impl Def {
    pub fn function(ident: Ident, expr: Expr) -> Self {
        Self::Fn {
            ident,
            expr,
            ty: None,
        }
    }

    pub fn ty(ident: Ident, ty: TypeExpr) -> Self {
        Self::Type { ident, ty }
    }

    pub fn typed_function(ident: Ident, ty: Option<TypeExpr>, expr: Expr) -> Self {
        Self::Fn { ident, ty, expr }
    }

    pub fn type_binding(ident: Ident, ty: TypeExpr) -> Self {
        Self::Binding { ident, ty }
    }
}


