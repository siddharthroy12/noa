use std::fs;

use crate::lox::{error::LoxError, parser::Parser, scanner::Scanner};
mod error;
mod expression;
mod parser;
mod scanner;
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
        match parser.generate_tree() {
            Err(err) => {
                return Err(Self::report_lox_error(err));
            }
            Ok(expression) => match expression.evaluate() {
                Ok(value) => {
                    println!("{}", value.to_string())
                }
                Err(err) => {
                    return Err(Self::report_lox_error(err));
                }
            },
        }
        return Ok(());
    }
    pub fn run_file(self: &mut Self, path: String) -> Result<(), String> {
        match fs::read_to_string(path) {
            Ok(content) => {
                return self.run(content);
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
