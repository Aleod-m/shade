mod bin_expr;
mod unary_expr;
use std::collections::HashMap;

pub use unary_expr::*;
mod op;
pub use op::*;
mod value;
pub use value::*;

use super::EvaluationError;

pub enum Expr {
    Unary(UnaryExpr),
    Value(Value),
    Ident(String),
}


impl Expr {
    pub fn unary(op: Op, expr: Expr) -> Self {
        if !op.is_unary_op() {
            panic!("Op is not unary!");
        }
       Expr::Unary(UnaryExpr{ op, expr: Box::new(expr)})
    }


    pub fn eval(&self, context: &mut HashMap<String, Value>) -> Result<Value, EvaluationError> {
        use Expr::*;
        match self {
            Unary(uexpr) => uexpr.eval(context),
            Value(val) => Ok(*val),
            Ident(name) => context
                .get(name)
                .map(|val| *val)
                .ok_or(EvaluationError::ValueNotPresentInContext(name.to_string())),
        }
    }
}
