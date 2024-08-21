use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::literal_value::LiteralValue;

#[derive(Clone, Debug)]
pub struct Environment {
    pub(crate) values: HashMap<String, LiteralValue>,
    pub(crate) constants: HashMap<String, bool>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // Create a new environment with no enclosing scope
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            constants: HashMap::new(), // Initialize the constants map
            enclosing: None,
        }
    }

    // Create a new environment with an enclosing scope
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            constants: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    // Define a new variable in the current environment
    pub fn define(&mut self, name: String, value: LiteralValue, is_const: bool) {
        self.values.insert(name.clone(), value);
        if is_const {
            self.constants.insert(name, true);
        }
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
        if let Some(is_const) = self.constants.get(name) {
            if *is_const {
                // Prevent reassignment to a constant
                println!("Cannot reassign to constant '{}'.", name);
                return false;
            }
        }

        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            println!("Failed to assign variable: '{}' with value '{}'", name, value.to_string());
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