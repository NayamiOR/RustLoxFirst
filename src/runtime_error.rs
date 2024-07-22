use crate::token::Token;

pub(crate) struct RuntimeError {
    pub(crate) token: Token,
    pub(crate) message: String,
}

impl RuntimeError {
    pub(crate) fn new(token: Token, message: String) -> RuntimeError {
        RuntimeError { token, message }
    }
}
