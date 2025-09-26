use std::sync::{Arc, Mutex};

use crate::noa::{environment::Environment, error::NoaTermination, types::Object};

pub fn str(arguments: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, NoaTermination> {
    Ok(Object::String(arguments.first().unwrap().to_string()))
}
