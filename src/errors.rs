use crate::token::{Token, TokenType};

#[derive(Debug, Clone, Copy)]
pub struct ParseError;

pub struct RuntimeError {
    message: String,
    token: Token,
}

impl RuntimeError {
    pub fn new(message: String, token: Token) -> Self {
        RuntimeError { message, token }
    }
}