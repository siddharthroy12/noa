use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::lox::{
    error::{LoxError, LoxTermination},
    token::Token,
    types::Object,
};
#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Arc<Mutex<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let values: HashMap<String, Object> = HashMap::new();
        return Environment {
            values: values,
            enclosing: None,
        };
    }
    pub fn snapshot(self: &Self) -> Result<Self, LoxTermination> {
        match self.enclosing.clone() {
            Some(e) => match e.lock() {
                Ok(mutex) => {
                    let enclose = mutex.snapshot()?;
                    return Ok(Self {
                        values: self.values.clone(),
                        enclosing: Some(Arc::new(Mutex::new(enclose))),
                    });
                }
                Err(_) => {
                    return Err(LoxTermination::Error(LoxError {
                        line: 0,
                        location: "N/A".to_owned(),
                        message: "Failed to lock environemnt".to_owned(),
                    }));
                }
            },
            _ => {}
        };
        return Ok(Self {
            values: self.values.clone(),
            enclosing: None,
        });
    }
    pub fn enclose(self: &mut Self, enclosing: Arc<Mutex<Environment>>) {
        self.enclosing = Some(enclosing);
    }
    pub fn define(self: &mut Self, identifier: String, value: Object) {
        self.values.insert(identifier, value);
    }
    pub fn assign(self: &mut Self, token: &Token, value: Object) -> Result<(), LoxTermination> {
        if !self.values.contains_key(&token.lexeme) {
            match &self.enclosing {
                Some(enclosing) => match enclosing.lock() {
                    Ok(mut enclosing) => return enclosing.assign(token, value),
                    Err(_) => {
                        panic!("Failed to get enclosing environment")
                    }
                },
                None => {
                    return Err(LoxTermination::Error(LoxError {
                        line: token.line,
                        location: token.lexeme.clone(),
                        message: format!("Unkown variable"),
                    }));
                }
            }
        }
        self.values.insert(token.lexeme.clone(), value);
        Ok(())
    }

    pub fn get(self: &mut Self, token: &Token) -> Result<Object, LoxTermination> {
        match self.values.get(&token.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => match enclosing.lock() {
                    Ok(mut enclosing) => return enclosing.get(token),
                    Err(_) => {
                        panic!("Failed to get enclosing environment")
                    }
                },
                None => Err(LoxTermination::Error(LoxError {
                    line: token.line,
                    location: token.lexeme.clone(),
                    message: format!("Unkown variable"),
                })),
            },
        }
    }
    pub fn get_by_string(self: &mut Self, key: String) -> Result<Object, ()> {
        match self.values.get(&key) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => match enclosing.lock() {
                    Ok(mut enclosing) => return enclosing.get_by_string(key),
                    Err(_) => {
                        panic!("Failed to get enclosing environment")
                    }
                },
                None => Err(()),
            },
        }
    }
}
