use crate::expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr, Visitor};
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

// TODO: 用模式匹配把visit_binary这些函数都换成模式匹配里的代码块，合并到一个函数里
impl Visitor<String> for ExprVisitor {
    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary_expr) => self.parenthesize(
                binary_expr.operator.lexeme.as_str(),
                vec![&binary_expr.left, &binary_expr.right],
            ),
            Expr::Grouping(grouping_expr) => {
                self.parenthesize("group", vec![&grouping_expr.expression])
            }
            Expr::Literal(literal_expr) => {
                if let Literal::Nil = literal_expr.value {
                    return String::from("nil");
                }
                literal_expr.value.to_string()
            }
            Expr::Unary(unary_expr) => {
                self.parenthesize(unary_expr.operator.lexeme.as_str(), vec![&unary_expr.right])
            }
        }
    }
}

// impl crate::expr::Visitor<String> for ExprVisitor {
//     fn visit_binary(&self, binary_expr: &BinaryExpr) -> String {
//         self.parenthesize(
//             binary_expr.operator.lexeme.as_str(),
//             vec![&binary_expr.left, &binary_expr.right],
//         )
//     }

//     fn visit_grouping(&self, grouping_expr: &GroupingExpr) -> String {
//         self.parenthesize("group", vec![&grouping_expr.expression])
//     }

//     fn visit_literal(&self, literal_expr: &LiteralExpr) -> String {
//         if let Literal::Nil = literal_expr.value {
//             return String::from("nil");
//         }
//         literal_expr.value.to_string()
//     }

//     fn visit_unary(&self, unary_expr: &UnaryExpr) -> String {
//         self.parenthesize(unary_expr.operator.lexeme.as_str(), vec![&unary_expr.right])
//     }
// }
