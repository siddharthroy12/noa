use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::lox::{error::LoxError, token::Token, types::Object};

pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Arc<Mutex<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Environment {
            values: HashMap::new(),
            enclosing: None,
        };
    }
    pub fn enclose(self: &mut Self, enclosing: Arc<Mutex<Environment>>) {
        self.enclosing = Some(enclosing);
    }
    pub fn define(self: &mut Self, token: &Token, value: Object) {
        self.values.insert(token.lexeme.clone(), value);
    }
    pub fn assign(self: &mut Self, token: &Token, value: Object) -> Result<(), LoxError> {
        if !self.values.contains_key(&token.lexeme) {
            match &self.enclosing {
                Some(enclosing) => match enclosing.lock() {
                    Ok(mut enclosing) => return enclosing.assign(token, value),
                    Err(_) => {
                        panic!("Failed to get enclosing environment")
                    }
                },
                None => {
                    return Err(LoxError {
                        line: token.line,
                        location: token.lexeme.clone(),
                        message: format!("Unkown variable"),
                    });
                }
            }
        }
        self.values.insert(token.lexeme.clone(), value);
        Ok(())
    }

    pub fn get(self: &mut Self, token: &Token) -> Result<Object, LoxError> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => match enclosing.lock() {
                    Ok(mut enclosing) => return enclosing.get(token),
                    Err(_) => {
                        panic!("Failed to get enclosing environment")
                    }
                },
                None => Err(LoxError {
                    line: token.line,
                    location: token.lexeme.clone(),
                    message: format!("Unkown variable"),
                }),
            },
        }
    }
}
