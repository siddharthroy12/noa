use std::fs;

use crate::lox::{parser::Parser, scanner::Scanner};
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
            return Err(self.report(err.line, err.location, err.message));
        }

        scanner.debug_print();

        let mut parser: Parser = Parser::new(scanner.tokens);
        match parser.generateTree() {
            Err(err) => {
                return Err(self.report(err.line, err.location, err.message));
            }
            Ok(expression) => {
                expression.print();
            }
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
    pub fn report(self: &mut Self, line: usize, place: String, message: String) -> String {
        return format!("[line \"{}\"] Error{}: {}", line, place, message);
    }
}
