use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;

pub struct PackageRegistry {
    packages: HashMap<String, Box<dyn Fn(Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String>>>,
}

impl PackageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            packages: HashMap::new(),
        };
        
        // Register built-in packages
        registry.register_builtin_packages();
        
        registry
    }

    pub fn load_package(&self, package_name: &str, parent_env: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String> {
        match self.packages.get(package_name) {
            Some(loader) => loader(parent_env),
            None => Err(format!("Package '{}' not found", package_name)),
        }
    }

    pub fn list_packages(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    fn register_builtin_packages(&mut self) {
        // Register standard library
        self.packages.insert("std".to_string(), Box::new(|parent_env| {
            crate::packages::std::load_std_package(parent_env)
        }));

        // Register IO package
        self.packages.insert("io".to_string(), Box::new(|parent_env| {
            crate::packages::io::load_io_package(parent_env)
        }));

        // Register Math package
        self.packages.insert("math".to_string(), Box::new(|parent_env| {
            crate::packages::math::load_math_package(parent_env)
        }));
    }
}