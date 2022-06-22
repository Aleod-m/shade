use std::{collections::HashMap, ops::Deref};

mod op;
pub use op::*;
mod unary_expr;
pub use unary_expr::*;
mod bin_expr;
pub use bin_expr::*;
mod value;
pub use value::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(String);

impl Deref for Ident {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Ident(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Value(Value),
    Ident(Ident),
}

impl Expr {
    pub fn unary(op: Op, expr: Expr) -> Self {
        if !op.is_unary_op() {
            panic!("Op is not unary!");
        }
        Expr::Unary(UnaryExpr {
            op,
            expr: Box::new(expr),
        })
    }

    pub fn binary(op: Op, lhs: Expr, rhs: Expr) -> Self {
        if !op.is_bin_op() {
            panic!("Op is not unary!");
        }
        Expr::Binary(BinaryExpr {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    pub fn value(val: impl Into<Value>) -> Self {
        Self::Value(val.into())
    }

    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value {
        use Expr::*;
        match self {
            Unary(uexpr) => uexpr.eval(context),
            Binary(bexpr) => bexpr.eval(context),
            Value(val) => val.eval(context),
            Ident(name) => {
                let ident_val = context.get(name).unwrap().clone();
                ident_val.eval(context)
            }
        }
    }
}

//impl TypeCheckVisitor for Expr {
//    fn ty_check(&self, context: &mut HashMap<String, Type>) -> Result<Type, TypeError> {
//        use Expr::*;
//        match self {
//            Unary(uexpr) => uexpr.ty_check(context),
//            Binary(bexpr) => bexpr.ty_check(context),
//            Value(val) => val.ty_check(context),
//            Ident(name) => context.get(name).map(|ty| *ty).ok_or(TypeError::TypedValueNotPresentInContext),
//        }
//    }
//}
