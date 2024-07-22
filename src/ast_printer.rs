use crate::expr::{Expr, Visitor};
use crate::token::Literal;

pub(crate) struct ExprVisitor;

impl ExprVisitor {
    pub(crate) fn print(&self, expr: &Expr) -> String {
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
    fn visit_binary(&self, left: &Expr, operator: &crate::token::Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), vec![left, right])
    }

    fn visit_grouping(&self, expression: &Expr) -> String {
        self.parenthesize("group", vec![expression])
    }

    fn visit_literal(&self, value: &crate::token::Literal) -> String {
        if let Literal::Nil = value {
            return String::from("nil");
        }
        value.to_string()
    }

    fn visit_unary(&self, operator: &crate::token::Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.as_str(), vec![right])
    }
}

pub(crate) fn print(expr: &Expr) -> String {
    expr.accept(&ExprVisitor)
}

fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let mut result = String::from("(");
    result.push_str(name);
    for expr in exprs {
        result.push(' ');
        result.push_str(&expr.accept(&ExprVisitor));
    }
    result.push(')');
    result
}
