use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct Environment {
    values: HashMap<String, Value>,      // 变量名到值的映射
    enclosing: Option<Box<Environment>>, // 外围环境
}

impl Environment {
    pub(crate) fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub(crate) fn new_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)?;
            return Ok(());
        }

        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", &name.lexeme),
        })
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(v) = self.values.get(&name.lexeme) {
            return Ok(v.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined variable '{}'.", &name.lexeme),
        })
    }
}
