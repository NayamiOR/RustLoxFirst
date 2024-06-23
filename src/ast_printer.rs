use crate::expr::{Expr, Visitor};
use crate::token::Literal;

pub struct ExprVisitor;

impl ExprVisitor {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut result = String::from("(");
        result.push_str(name);
        for expr in exprs {
            result.push(' ');
            result.push_str(&expr.accept(self));
        }
        result.push(')');
        result
    }
}

impl Visitor<String> for ExprVisitor {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(
                operator.lexeme.as_str(),
                vec![left.as_ref(), right.as_ref()],
            ),
            Expr::Grouping { expression } => self.parenthesize("group", vec![expression.as_ref()]),
            Expr::Literal { value } => {
                if let Literal::Nil = value {
                    return String::from("nil");
                }
                value.to_string()
            }
            Expr::Unary { operator, right } => {
                self.parenthesize(operator.lexeme.as_str(), vec![right.as_ref()])
            }
        }
    }
}
