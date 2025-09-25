use crate::lox::types::Object;

pub enum LoxTermination {
    Error(LoxError),
    Return(Object),
    Break,
    Continue,
}

pub struct LoxError {
    pub line: usize,
    pub location: String,
    pub message: String,
}
