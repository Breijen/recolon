use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::literal_value::LiteralValue;

#[derive(Clone, Debug)]
pub struct Environment {
    pub(crate) values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // Create a new environment with no enclosing scope
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    // Create a new environment with an enclosing scope
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    // Define a new variable in the current environment
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn define_top_level(&mut self, name: String, value: LiteralValue) {
        match &self.enclosing {
            None => self.define(name, value),
            Some(env) => env.borrow_mut().define_top_level(name, value),
        }
    }

    pub fn delete(&mut self) {

    }

    // Get the value of a variable, searching enclosing environments if necessary
    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        if let Some(val) = self.values.get(name) {
            return Some(val.clone());
        }
        if let Some(env) = &self.enclosing {
            return env.borrow().get(name);
        }
        None
    }

    // Assign a value to an existing variable, searching enclosing environments if necessary
    pub fn assign(&mut self, name: &str, value: LiteralValue) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            println!("Failed to define function: {} with value {}", name, value.to_string());
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_init() {
        let _environment = Environment::new();
    }
}