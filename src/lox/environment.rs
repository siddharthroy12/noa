use std::collections::HashMap;

use crate::lox::{environment, error::LoxError, token::Token, types::Object};

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: HashMap::new(),
            enclosing: None,
        };
    }
    pub fn enclose(self: &mut Self, enclosing: Environment) {
        self.enclosing = Some(Box::new(enclosing));
    }
    pub fn define(self: &mut Self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(self: &mut Self, token: Token) -> Result<Object, LoxError> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(LoxError {
                line: token.line,
                location: token.lexeme,
                message: format!("Unkown variable"),
            }),
        }
    }
}
