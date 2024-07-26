use crate::token::Token;

pub(crate) trait Visitor<R> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_grouping(&mut self, expression: &Expr) -> R;
    fn visit_literal(&mut self, value: &crate::token::Literal) -> R;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> R;
    fn visit_variable(&mut self, name: &Token) -> R;
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> R;
}

#[derive(Debug)]
pub(crate) enum Expr {
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
    Variable {
        name: Token
    },
    Assign{
        name: Token,
        value: Box<Expr>
    }
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Variable { name } => visitor.visit_variable(name),
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
        }
    }
}
