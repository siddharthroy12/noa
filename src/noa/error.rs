use crate::noa::types::{Number, Object};

pub enum NoaTermination {
    Error(NoaError),
    Return(Object),
    Break,
    Continue,
    Exit(Number),
}

pub struct NoaError {
    pub line: usize,
    pub location: String,
    pub message: String,
}
