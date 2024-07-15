use crate::token::Token;

pub trait Visitor<R> {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping(&self, expression: &Expr) -> R;
    fn visit_literal(&self, value: &crate::token::Literal) -> R;
    fn visit_unary(&self, operator: &Token, right: &Expr) -> R;
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
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
        }
    }
}
