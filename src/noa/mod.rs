use std::{
    fs,
    sync::{Arc, Mutex},
};

use crate::noa::{
    environment::Environment,
    error::NoaError,
    generic::{exit, len},
    interpreter::Interpreter,
    io::{input, print, println},
    parser::Parser,
    scanner::Scanner,
    string::{str, str_to_num},
    types::{Function, Number, Object},
};
mod environment;
mod error;
mod expression;
mod generic;
mod interpreter;
mod io;
mod number;
mod parser;
mod scanner;
mod statement;
mod string;
mod token;
mod types;
pub struct Noa {
    interpreter: Interpreter,
}

impl Noa {
    pub fn new() -> Self {
        return Noa {
            interpreter: Interpreter::new(),
        };
    }
    pub fn load_libray(self: &mut Self) {
        let env = Arc::new(Mutex::new(Environment::new()));
        // IO
        self.setup_global_object(
            "println".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["str".to_string()],
                body: None,
                callback: Some(println),
                environment: env.clone(),
            })),
        );
        self.setup_global_object(
            "print".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["str".to_string()],
                body: None,
                callback: Some(print),
                environment: env.clone(),
            })),
        );
        self.setup_global_object(
            "input".to_owned(),
            Object::Function(Box::new(Function {
                params: vec![],
                body: None,
                callback: Some(input),
                environment: env.clone(),
            })),
        );

        // String
        self.setup_global_object(
            "str".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["any".to_string()],
                body: None,
                callback: Some(str),
                environment: env.clone(),
            })),
        );
        self.setup_global_object(
            "str_to_num".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["str".to_string()],
                body: None,
                callback: Some(str_to_num),
                environment: env.clone(),
            })),
        );

        // Number

        // Generic
        self.setup_global_object(
            "len".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["any".to_string()],
                body: None,
                callback: Some(len),
                environment: env.clone(),
            })),
        );
        self.setup_global_object(
            "exit".to_owned(),
            Object::Function(Box::new(Function {
                params: vec!["num".to_string()],
                body: None,
                callback: Some(exit),
                environment: env.clone(),
            })),
        );
    }
    pub fn setup_global_object(self: &mut Self, identifier: String, object: Object) {
        self.interpreter.setup_global_object(identifier, object);
    }
    pub fn run(self: &mut Self, src: String) -> Result<Number, String> {
        let mut scanner = Scanner::new(src);
        if let Err(err) = scanner.scan_tokens() {
            return Err(Self::report_noa_error(err));
        }

        let mut parser: Parser = Parser::new(scanner.tokens);
        match parser.parse() {
            Err(err) => {
                return Err(Self::report_noa_error(err));
            }
            Ok(statements) => match self.interpreter.execute(statements) {
                Err(err) => {
                    return Err(Self::report_noa_error(err));
                }
                Ok(num) => {
                    return Ok(num);
                }
            },
        }
    }
    pub fn run_file(self: &mut Self, path: String) -> Result<(Number), String> {
        match fs::read_to_string(path) {
            Ok(content) => {
                let num = self.run(content)?;
                Ok(num)
            }
            Err(_) => return Err("Failed to read the file".to_string()),
        }
    }
    pub fn report_noa_error(error: NoaError) -> String {
        return format!(
            "[line \"{}\"] Error at '{}': {}",
            error.line, error.location, error.message
        );
    }
}
