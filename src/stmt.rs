use crate::expr::Expr;
use crate::token::Token;

pub(crate) trait Visitor<R> {
    fn visit_expression(&mut self, expr: &Expr) -> R;
    fn visit_print(&mut self, expr: &Expr) -> R;
    fn visit_var(&mut self, name: &Token, initializer: Option<&Expr>) -> R;
    fn visit_block(&mut self, statements: &Vec<Stmt>) -> R;
}

pub(crate) enum Stmt {
    Expression {
        expression: Box<Expr>
    },
    Print {
        expression: Box<Expr>
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Block{
        statements: Vec<Stmt>
    }
}

impl Stmt {
    pub(crate) fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Stmt::Print { expression } => visitor.visit_print(expression),
            Stmt::Expression { expression } => visitor.visit_expression(expression),
            Stmt::Var { name, initializer } => visitor.visit_var(name, initializer.as_deref()),
            Stmt::Block { statements } => visitor.visit_block(statements),
        }
    }
}