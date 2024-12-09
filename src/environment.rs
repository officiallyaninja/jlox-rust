use crate::literal::Literal;
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment<'p> {
    variables: HashMap<String, Literal>,
    parent: Option<&'p Environment<'p>>,
}
impl<'p> Environment<'p> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_parent(env: &'p Environment) -> Environment<'p> {
        Self {
            parent: Some(env),
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
