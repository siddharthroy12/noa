use std::sync::{Arc, Mutex};

use crate::noa::{environment::Environment, error::NoaTermination, types::Object};

// IO
pub fn print(
    arguments: &Vec<Object>,
    _: Arc<Mutex<Environment>>,
) -> Result<Object, NoaTermination> {
    for arg in arguments {
        print!("{}", arg.to_string());
    }
    println!("");
    Ok(Object::Nil)
}
