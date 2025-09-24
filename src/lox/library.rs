use std::sync::{Arc, Mutex};

use crate::lox::{environment::Environment, error::LoxError, types::Object};

// IO
pub fn print(arguments: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, LoxError> {
    for arg in arguments {
        print!("{}", arg.to_string());
    }
    println!("");
    Ok(Object::Nil)
}
