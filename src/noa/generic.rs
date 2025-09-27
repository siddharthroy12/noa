use std::{
    fmt::format,
    sync::{Arc, Mutex},
};

use crate::noa::{
    environment::Environment,
    error::{NoaError, NoaTermination},
    types::Object,
};

pub fn len(arguments: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, NoaTermination> {
    match arguments.first().unwrap() {
        Object::String(str) => Ok(Object::Number(str.len() as f64)),
        Object::Table(mutex) => match mutex.lock() {
            Ok(table) => Ok(Object::Number(table.values.len() as f64)),
            Err(_) => {
                return Err(NoaTermination::Error(NoaError {
                    line: 0,
                    location: "N/A".to_owned(),
                    message: "Failed to lock table".to_owned(),
                }));
            }
        },
        _ => {
            return Err(NoaTermination::Error(NoaError {
                line: 0,
                location: "N/A".to_owned(),
                message: "len can only be called on strings and tables".to_owned(),
            }));
        }
    }
}

pub fn exit(arguments: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, NoaTermination> {
    let first = arguments.first().unwrap();

    match first {
        Object::Number(num) => Err(NoaTermination::Exit(*num)),
        _ => {
            return Err(NoaTermination::Error(NoaError {
                line: 0,
                location: first.to_string(),
                message: format!("{} is not a number", first.to_string()),
            }));
        }
    }
}
