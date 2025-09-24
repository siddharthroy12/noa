use std::sync::{Arc, Mutex};

use crate::lox::{
    environment::{self, Environment},
    error::LoxError,
    statement::Statement,
    token::TokenType,
    types::Object,
};

pub struct Interpreter {
    environment: Arc<Mutex<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            environment: Arc::new(Mutex::new(Environment::new())),
        };
    }
    pub fn setup_global_object(self: &mut Self, name: String, value: Object) {
        match self.environment.lock() {
            Ok(mut env) => {
                env.define(name, value);
            }
            Err(_) => panic!("Failed to set global object {}", name),
        }
    }
    pub fn execute(self: &mut Self, statements: Vec<Statement>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(self.environment.clone())?;
        }

        Ok(())
    }
}
