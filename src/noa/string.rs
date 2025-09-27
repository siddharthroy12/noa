use std::sync::{Arc, Mutex};

use crate::noa::{
    environment::Environment,
    error::{NoaError, NoaTermination},
    types::{Number, Object},
};

pub fn str(arguments: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, NoaTermination> {
    Ok(Object::String(arguments.first().unwrap().to_string()))
}

pub fn str_to_num(
    arguments: &Vec<Object>,
    _: Arc<Mutex<Environment>>,
) -> Result<Object, NoaTermination> {
    let first = arguments.first().unwrap();
    match first {
        Object::String(str) => {
            let num = str.parse::<Number>();

            match num {
                Ok(num) => Ok(Object::Number(num)),
                Err(_) => Ok(Object::Nil),
            }
        }
        _ => {
            return Err(NoaTermination::Error(NoaError {
                line: 0,
                location: first.to_string(),
                message: format!("Cannot convert {} into number", first.to_string()),
            }));
        }
    }
}
