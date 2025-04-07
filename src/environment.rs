use crate::literal::Literal;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: Vec<HashMap<String, Literal>>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
        }
    }
    pub fn push_scope(&mut self) {
        self.variables.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        assert_ne!(self.variables.len(), 1, "cannot pop scope when scope is 1");
        self.variables.pop();
    }

    pub fn scope(&self) -> usize {
        self.variables.len()
    }
    pub fn insert(&mut self, key: String, value: Literal) -> Option<Literal> {
        self.variables
            .last_mut()
            .expect("should have at least one element")
            .insert(key, value)
    }
    pub fn get(&self, key: &str) -> Option<&Literal> {
        for map in self.variables.iter().rev() {
            let value = map.get(key);
            if value.is_some() {
                return value;
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Literal> {
        for map in self.variables.iter_mut().rev() {
            let value = map.get_mut(key);
            if value.is_some() {
                return value;
            }
        }
        None
    }
}
