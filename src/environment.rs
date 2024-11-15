use crate::literal::Literal;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    variables: HashMap<String, Literal>,
    parent: Option<Box<Environment>>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_parent(env: Environment) -> Self {
        Self {
            parent: Some(Box::new(env)),
            ..Self::default()
        }
    }
    pub fn insert(&mut self, key: String, value: Literal) -> Option<Literal> {
        self.variables.insert(key, value)
    }
    pub fn get(&self, key: &str) -> Option<&Literal> {
        match (self.variables.get(key), &self.parent) {
            (Some(value), _) => Some(value),
            (None, Some(parent)) => parent.get(key),
            (None, None) => None,
        }
    }
}
