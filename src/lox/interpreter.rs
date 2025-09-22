use std::sync::{Arc, Mutex};

use crate::lox::{environment::Environment, error::LoxError, statement::Statement};

pub struct Interpreter {
    environment: Arc<Mutex<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            environment: Arc::new(Mutex::new(Environment::new())),
        };
    }
    pub fn execute(self: &mut Self, statements: Vec<Statement>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(self.environment.clone())?;
        }

        Ok(())
    }
}
