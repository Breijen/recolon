use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;
use crate::packages::registry::PackageRegistry;

pub struct PackageManager {
    registry: PackageRegistry,
    loaded_packages: HashMap<String, Rc<RefCell<Environment>>>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            registry: PackageRegistry::new(),
            loaded_packages: HashMap::new(),
        }
    }

    pub fn load_package(&mut self, package_name: &str, environment: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String> {
        // Check if package is already loaded
        if let Some(package_env) = self.loaded_packages.get(package_name) {
            return Ok(package_env.clone());
        }

        // Load package from registry
        let package_env = self.registry.load_package(package_name, environment)?;
        
        // Cache the loaded package
        self.loaded_packages.insert(package_name.to_string(), package_env.clone());
        
        Ok(package_env)
    }

    pub fn get_package(&self, package_name: &str) -> Option<Rc<RefCell<Environment>>> {
        self.loaded_packages.get(package_name).cloned()
    }

    pub fn setup_default_imports(&mut self, environment: Rc<RefCell<Environment>>) -> Result<(), String> {
        // Load standard library automatically
        let std_env = self.load_package("std", environment.clone())?;
        environment.borrow_mut().define("std".to_string(), LiteralValue::Namespace(std_env), false);

        // Load io module
        let io_env = self.load_package("io", environment.clone())?;
        environment.borrow_mut().define("io".to_string(), LiteralValue::Namespace(io_env), false);

        // Load math module  
        let math_env = self.load_package("math", environment.clone())?;
        environment.borrow_mut().define("math".to_string(), LiteralValue::Namespace(math_env), false);

        Ok(())
    }

    pub fn list_available_packages(&self) -> Vec<String> {
        self.registry.list_packages()
    }
}