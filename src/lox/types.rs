use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::lox::{environment::Environment, error::LoxError, statement::BlockStatement};

pub type Number = f64;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub body: Option<BlockStatement>,
    pub callback: Option<
        fn(
            arguments: &Vec<Object>,
            environment: Arc<Mutex<Environment>>,
        ) -> Result<Object, LoxError>,
    >,
}

impl Function {
    pub fn call(
        self: &Self,
        arguments: Vec<Object>,
        environment: Arc<Mutex<Environment>>,
    ) -> Result<Object, LoxError> {
        match self.callback {
            Some(callback) => {
                return callback(&arguments, environment);
            }
            None => {}
        }

        match &self.body {
            Some(block) => {
                for statement in &block.statements {
                    statement.execute(environment.clone())?;
                }
            }
            None => {}
        }

        return Ok(Object::Nil);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(Number),
    String(String),
    Bool(bool),
    Function(Box<Function>),
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
        }
    }
}
