mod assign_stmt;
use std::collections::HashMap;

pub use assign_stmt::*;

use super::{Expr, Value, Ident};

pub enum Stmt {
    Assign(AssignStmt),
    Expr(Expr),
}

impl Stmt {
     pub fn assign(ident: Ident, expr: Expr) -> Self {
        Self::Assign(AssignStmt{
            ident,
            expr,
        })
    }

    pub fn eval(&self, context: &mut HashMap<Ident, Value>) -> Value {
        use Stmt::*;
        match self {
            Assign(stmt) => stmt.eval(context),
            Expr(expr) => expr.eval(context),
        }
    }
}
