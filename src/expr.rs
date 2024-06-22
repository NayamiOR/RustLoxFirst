/*
     "Binary   : Expr left, Token operator, Expr right",
     "Grouping : Expr expression",
     "Literal  : Object value",
     "Unary    : Token operator, Expr right"
*/

use crate::ast_printer::ExprVisitor;
use crate::token::{Literal, Token};
use std::fmt::Display;

pub trait Visitor<R> {
    // fn visit_binary(&self, binary_expr: &BinaryExpr) -> R;
    // fn visit_grouping(&self, grouping_expr: &GroupingExpr) -> R;
    // fn visit_literal(&self, literal_expr: &LiteralExpr) -> R;
    // fn visit_unary(&self, unary_expr: &UnaryExpr) -> R;
    fn visit(&self, expr: &Expr) -> R;
}

pub struct BinaryExpr {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

pub struct GroupingExpr {
    pub(crate) expression: Expr,
}

pub struct LiteralExpr {
    pub(crate) value: Literal,
}

pub struct UnaryExpr {
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

pub enum Expr {
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(Box<LiteralExpr>),
    Unary(Box<UnaryExpr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(binary) => write!(
                f,
                "{} {} {}",
                binary.operator.lexeme, binary.left, binary.right
            ),
            Expr::Grouping(grouping) => write!(f, "{}", grouping.expression),
            Expr::Literal(literal) => write!(f, "{}", literal.value),
            Expr::Unary(unary) => write!(f, "{} {}", unary.operator.lexeme, unary.right),
        }
    }
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        // match self {
        //     Expr::Binary(binary) => visitor.visit_binary(binary),
        //     Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
        //     Expr::Literal(literal) => visitor.visit_literal(literal),
        //     Expr::Unary(unary) => visitor.visit_unary(unary),
        // }
        visitor.visit(self)
    }
}

// accept(visitor){
//     switch(this){
//         Class 1: visitor.a(this),
//         Class 2: visitor.b(this),
//         ...
//         Class X: visitor.c(this),
//     }
// }