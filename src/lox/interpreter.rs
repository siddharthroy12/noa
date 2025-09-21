use crate::lox::{environment::Environment, error::LoxError, statement::Statement};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        return Interpreter {
            environment: Environment::new(),
        };
    }
    pub fn execute(self: &mut Self, statements: Vec<Statement>) -> Result<(), LoxError> {
        for statement in statements {
            statement.execute(&mut self.environment)?;
        }

        Ok(())
    }
}
