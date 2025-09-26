use std::sync::{Arc, Mutex};

use crate::noa::{
    Noa,
    environment::{self, Environment},
    error::NoaError,
    statement::Statement,
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
    pub fn execute(self: &mut Self, statements: Vec<Statement>) -> Result<(), NoaError> {
        for statement in statements {
            match statement.execute(self.environment.clone()) {
                Err(e) => match e {
                    crate::noa::error::NoaTermination::Error(noa_error) => return Err(noa_error),
                    super::error::NoaTermination::Return(_) => {
                        return Err(NoaError {
                            line: 0,
                            location: String::from("return"),
                            message: String::from("return can only be used inside a function"),
                        });
                    }
                    super::error::NoaTermination::Break => {
                        return Err(NoaError {
                            line: 0,
                            location: String::from("break"),
                            message: String::from("break can only be used loops"),
                        });
                    }
                    super::error::NoaTermination::Continue => {
                        return Err(NoaError {
                            line: 0,
                            location: String::from("continue"),
                            message: String::from("continue can only be used loops"),
                        });
                    }
                },
                _ => {}
            }
        }

        Ok(())
    }
}
