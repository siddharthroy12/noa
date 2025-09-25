use std::sync::{Arc, Mutex};

use crate::lox::{environment::Environment, error::LoxTermination, types::Object};

// IO
pub fn print(
    arguments: &Vec<Object>,
    _: Arc<Mutex<Environment>>,
) -> Result<Object, LoxTermination> {
    for arg in arguments {
        print!("{}", arg.to_string());
    }
    println!("");
    Ok(Object::Nil)
}
