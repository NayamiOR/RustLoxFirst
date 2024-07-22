use crate::expr::{Expr, Visitor};
use crate::runtime_error::RuntimeError;
use crate::token::{Literal, Token};
use crate::value::Value;
use crate::token_type::TokenType;
use crate::value::Value::*;
use crate::Lox;

pub(crate) struct Interpreter;

impl Visitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;

        match operator.token_type {
            TokenType::MINUS => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value - right_value)
            }
            TokenType::PLUS => {
                match (left_value, right_value) {
                    (Number(l), Number(r)) => Ok(Number(l + r)),
                    (String(l), String(r)) => Ok(String(format!("{}{}", l, r))),
                    _ => Err(RuntimeError::new(operator.clone(), "Operands must be two numbers or two strings.".to_string())),
                }
            }
            TokenType::SLASH => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value / right_value)
            }
            TokenType::STAR => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(left_value * right_value)
            }
            TokenType::GREATER => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value > right_value))
            }
            TokenType::GREATER_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value >= right_value))
            }
            TokenType::LESS => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value < right_value))
            }
            TokenType::LESS_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value <= right_value))
            }
            TokenType::BANG_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value != right_value))
            }
            TokenType::EQUAL_EQUAL => {
                Self::check_number_operands(operator, &left_value, &right_value)?;
                Ok(Boolean(left_value == right_value))
            }
            _ => { Ok(Nil) }
        }
    }

    fn visit_grouping(&self, expression: &Expr) -> Result<Value, RuntimeError> {
        self.evaluate(expression)
    }

    fn visit_literal(&self, value: &Literal) -> Result<Value, RuntimeError> {
        match value {
            Literal::String(s) => Ok(String(s.clone())),
            Literal::Number(n) => Ok(Number(*n)),
            Literal::Bool(b) => Ok(Boolean(*b)),
            Literal::Nil => Ok(Nil),
        }
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let right_value = self.evaluate(right)?;
        match operator.token_type {
            TokenType::MINUS => {
                Self::check_number_operand(operator, &right_value)?;
                Ok(-right_value)
            }
            TokenType::BANG => {
                Ok(!right_value)
            }
            _ => unreachable!()
        }
    }
}

impl Interpreter {
    pub(crate) fn interpret(&self, expr: &Expr) {
        match self.evaluate(expr) {
            Ok(value) => {
                println!("{}", value);
            }
            Err(e) => {
                Lox::runtime_error(e);
            }
        }
    }
    fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    fn check_number_operand(operator: &Token, operand: &Value) -> Result<(), RuntimeError> {
        if let Number(_) = operand {
            return Ok(());
        }
        Err(RuntimeError::new(operator.clone(), "Operand must be a number.".to_string()))
    }

    fn check_number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(), RuntimeError> {
        if let (Number(_), Number(_)) = (left, right) {
            return Ok(());
        }
        Err(RuntimeError::new(operator.clone(), "Operands must be numbers.".to_string()))
    }
}