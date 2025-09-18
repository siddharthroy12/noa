use std::fs;

use crate::lox::scanner::Scanner;
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
    pub fn run(self: &Self, src: String) -> Result<String, String> {
        let mut scanner = Scanner::new(src);
        scanner.scan_tokens()?;
        scanner.debug_print();
        return Ok("".to_string());
    }
    pub fn run_file(self: &Self, path: String) -> Result<String, String> {
        match fs::read_to_string(path) {
            Ok(content) => {
                return self.run(content);
            }
            Err(_) => return Err("Failed to read the file".to_string()),
        }
    }
    pub fn report(self: &mut Self, line: i32, place: String, message: String) -> String {
        return format!("[line \"{}\"] Error {} : {} ", line, place, message);
    }
}
