use std::any::Any;

use crate::lox::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<TokenType>,
}
