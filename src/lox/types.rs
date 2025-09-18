use std::fmt;

pub enum Object {
    Int(i32),
    String(String),
    Bool(bool),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(i) => write!(f, "{}", i),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Bool(b) => write!(f, "{}", b),
        }
    }
}
