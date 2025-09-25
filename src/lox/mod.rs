use std::fs;

use crate::lox::{
    error::LoxError,
    interpreter::Interpreter,
    library::print,
    parser::Parser,
    scanner::Scanner,
    types::{Function, Object},
};
mod environment;
mod error;
mod expression;
mod interpreter;
mod library;
mod parser;
mod scanner;
mod statement;
mod token;
mod types;
pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        return Lox {
            interpreter: Interpreter::new(),
        };
    }
    pub fn load_libray(self: &mut Self) {
        self.setup_global_object(
            "print".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["str".to_string()],
                body: None,
                callback: Some(print),
            })),
        );
    }
    pub fn setup_global_object(self: &mut Self, identifier: String, object: Object) {
        self.interpreter.setup_global_object(identifier, object);
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
            Ok(statements) => match self.interpreter.execute(statements) {
                Err(err) => {
                    return Err(Self::report_lox_error(err));
                }
                _ => {}
            },
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
            "[line \"{}\"] Error at '{}': {}",
            error.line, error.location, error.message
        );
    }
}
