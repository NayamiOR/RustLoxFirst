use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::Lox;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_token(&[VAR]) {
            match self.var_declaration() {
                Ok(stmt) => {
                    return Some(stmt);
                }
                Err(_) => self.synchronize(),
            }
        }
        match self.statement() {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[PRINT]) {
            return self.print_statement();
        }
        if self.match_token(&[LEFT_BRACE]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(IDENTIFIER, "Expect variable name.".to_string())?;
        let mut initializer = None;
        if self.match_token(&[EQUAL]) {
            initializer = Some(Box::new(self.expression()?));
        }
        self.consume(
            SEMICOLON,
            "Expect ';' after variable declaration.".to_string(),
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after expression.".to_string())?;
        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&RIGHT_BRACE) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        self.consume(RIGHT_BRACE, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_token(&[EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }
            return Err(Self::error(
                equals,
                "Invalid assignment target.".to_string(),
            ));
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and();

        while self.match_token(&[OR]) {
            let operator = self.previous();
            let right = self.and();
            expr = Ok(Expr::Logical {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality();

        while self.match_token(&[AND]) {
            let operator = self.previous();
            let right = self.equality();
            expr = Ok(Expr::Logical {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison();
        while self.match_token(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term();
        while self.match_token(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor();
        while self.match_token(&[MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary();
        while self.match_token(&[SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Ok(Expr::Binary {
                left: Box::new(expr?),
                operator,
                right: Box::new(right?),
            });
        }
        expr
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[FALSE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(false),
            });
        }
        if self.match_token(&[TRUE]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(true),
            });
        }
        if self.match_token(&[NIL]) {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }
        if self.match_token(&[NUMBER, STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone().unwrap(),
            });
        }
        if self.match_token(&[IDENTIFIER]) {
            return Ok(Expr::Variable {
                name: self.previous(),
            });
        }
        if self.match_token(&[LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(RIGHT_PAREN, "Expect ')' after expression.".to_string())?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }
        Err(Self::error(self.peek(), "Expect expression.".to_string()))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParseError> {
        if !self.check(&token_type) {
            return Err(Self::error(self.peek(), message));
        }
        Ok(self.advance())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(token: Token, message: String) -> ParseError {
        Lox::error_at_token(token, message);
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return;
            }
            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => (),
            }
            self.advance();
        }
    }
}

#[derive(Debug)]
pub(crate) struct ParseError;
