use std::{
    collections::HashMap,
    fmt::{self, Error},
    sync::{Arc, Mutex},
};

use crate::noa::{environment::Environment, error::NoaTermination, statement::Statement};

pub type Number = f64;

#[derive(Debug, Clone)]
pub struct Function {
    pub body: Option<Box<Statement>>,
    pub params: Vec<String>,
    pub environment: Arc<Mutex<Environment>>,
    pub callback: Option<
        fn(
            arguments: &Vec<Object>,
            environment: Arc<Mutex<Environment>>,
        ) -> Result<Object, NoaTermination>,
    >,
}

impl Function {
    pub fn call(self: &Self, arguments: Vec<Object>) -> Result<Object, NoaTermination> {
        let mut environment = Environment::new();
        environment.enclose(self.environment.clone());
        for (i, arg) in arguments.iter().enumerate() {
            environment.define(self.params[i].clone(), arg.clone());
        }
        let mut environment = Arc::new(Mutex::new((environment)));
        match self.callback {
            Some(callback) => {
                return callback(&arguments, environment);
            }
            None => {}
        }

        match &self.body {
            Some(block) => match block.execute(environment) {
                Err(e) => match e {
                    NoaTermination::Return(object) => {
                        return Ok(object);
                    }
                    _ => {
                        return Err(e);
                    }
                },
                _ => {}
            },
            None => {}
        }

        return Ok(Object::Nil);
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    pub values: HashMap<String, Object>,
}

impl Table {
    pub fn get_value(self: &Self, key: String) -> Object {
        match self.values.get(&key) {
            Some(value) => {
                return value.clone();
            }
            None => Object::Nil, // Somehow this is returned
        }
    }
    pub fn set_value(self: &mut Self, key: String, value: Object) {
        self.values.insert(key, value);
    }
}

#[derive(Debug, Clone)]
pub enum Object {
    Number(Number),
    String(String),
    Bool(bool),
    Function(Box<Function>),
    Table(Arc<Mutex<Table>>),
    Nil,
}

impl Object {
    pub fn is_truthy(self: &Self) -> bool {
        match self {
            Object::Number(num) => {
                return *num == 0.0;
            }
            Object::String(str) => return !str.is_empty(),
            Object::Bool(b) => {
                return *b;
            }
            Object::Nil => {
                return false;
            }
            Object::Function(_) => return true,
            Object::Table(_) => return true,
        }
    }
    pub fn is_equal(self: &Self, comp: &Object) -> bool {
        if let Object::Nil = self
            && let Object::Nil = comp
        {
            return true;
        }
        if let Object::Nil = self {
            return false;
        }
        if let Object::Nil = comp {
            return false;
        }
        match self {
            Object::Number(num1) => match comp {
                Object::Number(num2) => {
                    return num1 == num2;
                }
                _ => {
                    return false;
                }
            },
            Object::String(str) => match comp {
                Object::String(str2) => {
                    return str2 == str;
                }

                _ => {
                    return false;
                }
            },
            Object::Bool(b) => match comp {
                Object::Bool(b2) => {
                    return b == b2;
                }

                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }
    pub fn is_greater(self: &Self, comp: &Object) -> bool {
        match self {
            Object::Number(num1) => match comp {
                Object::Number(num2) => {
                    return num1 > num2;
                }
                _ => {
                    return false;
                }
            },
            Object::String(str) => match comp {
                Object::String(str2) => {
                    return str2 > str;
                }

                _ => {
                    return false;
                }
            },
            Object::Bool(b) => match comp {
                Object::Bool(b2) => {
                    return b > b2;
                }

                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }
    pub fn is_less(self: &Self, comp: &Object) -> bool {
        match self {
            Object::Number(num1) => match comp {
                Object::Number(num2) => {
                    return num1 < num2;
                }
                _ => {
                    return false;
                }
            },
            Object::String(str) => match comp {
                Object::String(str2) => {
                    return str2 < str;
                }

                _ => {
                    return false;
                }
            },
            Object::Bool(b) => match comp {
                Object::Bool(b2) => {
                    return b < b2;
                }

                _ => {
                    return false;
                }
            },
            _ => {
                return false;
            }
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(i) => write!(f, "{}", i),
            Object::String(s) => write!(f, "{}", s),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
            Object::Function(_) => write!(f, "[Function]"),
            Object::Table(table) => {
                write!(f, "{{")?;
                match table.lock() {
                    Ok(mutex) => {
                        for (key, val) in mutex.values.iter() {
                            write!(f, "{}:{},", key, val.to_string())?;
                        }
                    }
                    Err(_) => return Err(std::fmt::Error),
                }
                write!(f, "}}")?;
                Ok(())
            }
        }
    }
}
