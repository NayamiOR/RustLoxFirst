use std::collections::HashMap;
use once_cell::unsync::Lazy;
use crate::Lox;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use crate::token::Literal;

const Keywords: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut mp = HashMap::new();
    mp.insert(String::from("and"), AND);
    mp.insert(String::from("class"), CLASS);
    mp.insert(String::from("else"), ELSE);
    mp.insert(String::from("false"), FALSE);
    mp.insert(String::from("for"), FOR);
    mp.insert(String::from("fun"), FUN);
    mp.insert(String::from("if"), IF);
    mp.insert(String::from("nil"), NIL);
    mp.insert(String::from("or"), OR);
    mp.insert(String::from("print"), PRINT);
    mp.insert(String::from("return"), RETURN);
    mp.insert(String::from("super"), SUPER);
    mp.insert(String::from("this"), THIS);
    mp.insert(String::from("true"), TRUE);
    mp.insert(String::from("var"), VAR);
    mp.insert(String::from("while"), WHILE);
    mp
});

pub struct Scanner {
    source: String,
    tokens:Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens:Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self)->Vec<Token>{
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, String::from(""), None, self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i32
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LEFT_PAREN),
            ')' => self.add_token(RIGHT_PAREN),
            '{' => self.add_token(LEFT_BRACE),
            '}' => self.add_token(RIGHT_BRACE),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                let token_type = if self.match_char('=') { BANG_EQUAL } else { BANG };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') { EQUAL_EQUAL } else { EQUAL };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') { LESS_EQUAL } else { LESS };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') { GREATER_EQUAL } else { GREATER };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c if Scanner::is_digit(c) => self.number(),
            c if Scanner::is_alpha(c) => self.identifier(),
            _ => {
                Lox::error_at_line(self.line, "Unexpected character.".to_string())
            }
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start as usize..self.current as usize];
        let token_type = *Keywords.get(text).unwrap_or(&IDENTIFIER);
        match token_type {
            TRUE => self.add_token_with_literal(TRUE, Some(Literal::Bool(true))),
            FALSE => self.add_token_with_literal(FALSE, Some(Literal::Bool(false))),
            NIL => self.add_token_with_literal(NIL, Some(Literal::Nil)),
            _ => self.add_token(token_type),
        }
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Lox::error_at_line(self.line, "Unterminated string.".to_string());
            return;
        }

        self.advance();

        let value = &self.source[self.start as usize + 1..self.current as usize - 1];
        self.add_token_with_literal(STRING, Some(Literal::String(value.to_string())));
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start as usize..self.current as usize];
        self.add_token_with_literal(NUMBER, Some(Literal::Number(value.parse().unwrap())));
    }

    // 判断当前字符是否为expected，如果是，current指针后移一位
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    // 查看当前字符，但不移动current指针
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    // 预览下一个字符
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as i32 {
            return '\0';
        }
        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    // 判断是否是字母
    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    // 判断是否是字母或数字
    fn is_alphanumeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    // 判断是否是数字
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    // 查看当前字符并将current指针后移一位
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth((self.current - 1) as usize).unwrap()
    }

    // 添加token
    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(token_type, text.to_string(), None, self.line));
    }

    // 添加带有字面量的token
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(token_type, text.to_string(), literal, self.line));
    }
}

