use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;

pub mod time;
pub mod console;

pub fn load_std_package(parent_env: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String> {
    let std_env = Rc::new(RefCell::new(Environment::new_with_enclosing(parent_env)));

    // Load time functions
    time::register_time_functions(&std_env);
    
    // Load console functions
    console::register_console_functions(&std_env);

    Ok(std_env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_package_loading() {
        let parent_env = Rc::new(RefCell::new(Environment::new()));
        let std_env = load_std_package(parent_env).unwrap();
        
        // Test that clock function is available
        assert!(std_env.borrow().get("clock").is_some());
        
        // Test that wait_ms function is available
        assert!(std_env.borrow().get("wait_ms").is_some());
        
        // Test that color_console function is available
        assert!(std_env.borrow().get("color_console").is_some());
    }
}