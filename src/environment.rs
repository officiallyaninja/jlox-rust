use crate::literal::Literal;
use std::collections::HashMap;

pub struct Environment {
    pub variables: HashMap<String, Literal>,
    pub output: Vec<String>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            output: Vec::new(),
        }
    }
}
