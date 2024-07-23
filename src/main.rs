mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;
mod parser;
mod value;
mod interpreter;
mod runtime_error;

use scanner::Scanner;
use token::Token;
use crate::interpreter::Interpreter;

struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}

static mut LOX: Lox = Lox { had_error: false, had_runtime_error: false, interpreter: Interpreter };


fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => Lox::run_prompt().unwrap(),
        2 => Lox::run_file(args[1].clone()).unwrap(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

impl Lox {
    pub(crate) fn run_file(path: String) -> Result<(), std::io::Error> {
        let source = std::fs::read_to_string(path)?;
        Self::run(source);
        if unsafe { LOX.had_error } {
            std::process::exit(65);
        }
        if unsafe { LOX.had_runtime_error } {
            std::process::exit(70);
        }
        Ok(())
    }

    pub(crate) fn run_prompt() -> Result<(), std::io::Error> {
        loop {
            print!("> ");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            Self::run(line);
            unsafe {
                LOX.had_error = false;
            }
        }
    }

    pub(crate) fn run(source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = parser::Parser::new(tokens);
        let expression = parser.parse();
        if unsafe { LOX.had_error } {
            return;
        }

        unsafe {
            LOX.interpreter.interpret(&expression)
        }
    }

    pub(crate) fn error_at_line(line: i32, message: String) {
        Self::report(line, "".to_string(), message);
    }

    pub(crate) fn report(line: i32, location: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        unsafe {
            LOX.had_error = true;
        }
    }

    pub(crate) fn error_at_token(token: Token, message: String) {
        if token.token_type == token_type::TokenType::EOF {
            Self::report(token.line, " at end".to_string(), message);
        } else {
            Self::report(token.line, format!(" at '{}'", token.lexeme), message)
        }
    }

    pub(crate) fn runtime_error(error: runtime_error::RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        unsafe {
            LOX.had_runtime_error = true;
        }
    }
}
