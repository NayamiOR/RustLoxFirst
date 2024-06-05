use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token{
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme:String, literal: Option<Literal>, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}