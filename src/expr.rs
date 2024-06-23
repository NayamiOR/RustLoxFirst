use crate::token::Token;

pub trait Visitor<R> {
    fn visit(&self, expr: &Expr) -> R;
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: crate::token::Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        visitor.visit(self)
    }
}
