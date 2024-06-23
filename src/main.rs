mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;

use expr::Expr;
use scanner::Scanner;
use token::Token;

struct Lox {
    had_error: bool,
}

static mut LOX: Lox = Lox { had_error: false };

// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() > 2 {
//         println!("Usage: rlox [script]");
//         std::process::exit(64);
//     } else if args.len() == 2 {
//         Lox::run_file(args[1].clone());
//     } else {
//         Lox::run_prompt();
//     }
// }

fn main() {
    let expression=Expr::Binary{
        left: Box::new(Expr::Unary{
            operator: Token{
                token_type: token_type::TokenType::MINUS,
                lexeme: String::from("-"),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal{
                value: token::Literal::Number(123.0),
            }),
        }),
        operator: Token{
            token_type: token_type::TokenType::STAR,
            lexeme: String::from("*"),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping{
            expression: Box::new(Expr::Literal{
                value: token::Literal::Number(45.67),
            }),
        }),
    };
    let printer=ast_printer::ExprVisitor;
    println!("{}", printer.print(&expression));
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
        let tokens: &Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: i32, message: &str) {
        Self::report(line, "", message);
    }

    pub fn report(line: i32, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        unsafe {
            LOX.had_error = true;
        }
    }
}
