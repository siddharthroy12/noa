use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::noa::{environment::Environment, error::NoaTermination, types::Object};

pub fn print(
    arguments: &Vec<Object>,
    _: Arc<Mutex<Environment>>,
) -> Result<Object, NoaTermination> {
    for arg in arguments {
        print!("{}", arg.to_string());
    }
    std::io::stdout().flush().expect("Failed to flush stdout");
    Ok(Object::Nil)
}

pub fn println(
    arguments: &Vec<Object>,
    _: Arc<Mutex<Environment>>,
) -> Result<Object, NoaTermination> {
    for arg in arguments {
        print!("{}", arg.to_string());
    }
    println!("");
    Ok(Object::Nil)
}

pub fn input(_: &Vec<Object>, _: Arc<Mutex<Environment>>) -> Result<Object, NoaTermination> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => return Ok(Object::Nil),
    }
    input = input.replace("\n", "");
    Ok(Object::String(input))
}
