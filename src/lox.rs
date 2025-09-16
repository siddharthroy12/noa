use std::fs;

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        return Lox {};
    }
    pub fn run(self: &Self, str: String) -> Result<String, String> {
        return Ok(str);
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
