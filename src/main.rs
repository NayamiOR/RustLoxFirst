mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;
mod parser;

use scanner::Scanner;
use token::Token;

struct Lox {
    had_error: bool,
}

static mut LOX: Lox = Lox { had_error: false };


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        Lox::run_file(args[1].clone());
    } else {
        Lox::run_prompt();
    }
}

impl Lox {
    pub fn run_file(path: String) -> Result<(), std::io::Error> {
        let source = std::fs::read_to_string(path)?;
        Self::run(source);
        if unsafe { LOX.had_error } {
            std::process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt() -> Result<(), std::io::Error> {
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

    pub fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = parser::Parser::new(tokens);
        let expression = parser.parse();
        if unsafe { LOX.had_error } {
            return;
        }
        
        println!("{}",ast_printer::ExprVisitor.print(&expression));
    }

    pub fn error_at_line(line: i32, message: String) {
        Self::report(line, "".to_string(), message);
    }

    pub fn report(line: i32, location: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        unsafe {
            LOX.had_error = true;
        }
    }

    pub fn error_at_token(token: Token, message: String) {
        if token.token_type == token_type::TokenType::EOF {
            Self::report(token.line, " at end".to_string(), message);
        } else {
            Self::report(token.line, format!(" at '{}'", token.lexeme), message)
        }
    }
}
