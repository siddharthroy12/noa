use crate::noa::types::Object;

pub enum NoaTermination {
    Error(NoaError),
    Return(Object),
    Break,
    Continue,
}

pub struct NoaError {
    pub line: usize,
    pub location: String,
    pub message: String,
}
