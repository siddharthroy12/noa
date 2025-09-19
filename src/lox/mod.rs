use std::fs;

use crate::lox::{error::LoxError, parser::Parser, scanner::Scanner};
mod error;
mod expression;
mod parser;
mod scanner;
mod statement;
mod token;
mod types;

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        return Lox {};
    }
    pub fn run(self: &mut Self, src: String) -> Result<(), String> {
        let mut scanner = Scanner::new(src);
        if let Err(err) = scanner.scan_tokens() {
            return Err(Self::report_lox_error(err));
        }

        let mut parser: Parser = Parser::new(scanner.tokens);
        match parser.parse() {
            Err(err) => {
                return Err(Self::report_lox_error(err));
            }
            Ok(statements) => {
                for statement in statements {
                    match statement.execute() {
                        Err(err) => {
                            return Err(Self::report_lox_error(err));
                        }
                        _ => {}
                    }
                }
            }
        }
        return Ok(());
    }
    pub fn run_file(self: &mut Self, path: String) -> Result<(), String> {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.run(content)?;
                Ok(())
            }
            Err(_) => return Err("Failed to read the file".to_string()),
        }
    }
    pub fn report_lox_error(error: LoxError) -> String {
        return format!(
            "[line \"{}\"] Error{}: {}",
            error.line, error.location, error.message
        );
    }
}
