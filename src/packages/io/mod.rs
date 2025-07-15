use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::io::{self, Write};
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

pub fn load_io_package(parent_env: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String> {
    let io_env = Rc::new(RefCell::new(Environment::new_with_enclosing(parent_env)));

    // Register input functions
    io_env.borrow_mut().define("read_input".to_string(), LiteralValue::Callable {
        name: "read_input".to_string(),
        arity: 0,
        fun: Rc::new(read_input_impl),
    }, true);

    // Register file operations
    io_env.borrow_mut().define("open_file".to_string(), LiteralValue::Callable {
        name: "open_file".to_string(),
        arity: 1,
        fun: Rc::new(open_file_impl),
    }, true);

    io_env.borrow_mut().define("write_file".to_string(), LiteralValue::Callable {
        name: "write_file".to_string(),
        arity: 2,
        fun: Rc::new(write_file_impl),
    }, true);

    io_env.borrow_mut().define("file_exists".to_string(), LiteralValue::Callable {
        name: "file_exists".to_string(),
        arity: 1,
        fun: Rc::new(file_exists_impl),
    }, true);

    io_env.borrow_mut().define("delete_file".to_string(), LiteralValue::Callable {
        name: "delete_file".to_string(),
        arity: 1,
        fun: Rc::new(delete_file_impl),
    }, true);

    Ok(io_env)
}

fn read_input_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    
    LiteralValue::StringValue(input)
}

fn open_file_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("open_file requires exactly one argument.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return LiteralValue::StringValue("File path must be a string".to_string()),
    };

    match fs::read_to_string(filename) {
        Ok(contents) => LiteralValue::StringValue(contents),
        Err(e) => LiteralValue::StringValue(format!("Error reading file: {}", e)),
    }
}

fn write_file_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("write_file requires exactly 2 arguments: filename and content.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return LiteralValue::StringValue("File path must be a string".to_string()),
    };

    let content = match &args[1] {
        LiteralValue::StringValue(s) => s,
        _ => return LiteralValue::StringValue("File content must be a string".to_string()),
    };

    match fs::write(filename, content) {
        Ok(_) => LiteralValue::True,
        Err(e) => LiteralValue::StringValue(format!("Error writing to file: {}", e)),
    }
}

fn file_exists_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("file_exists requires exactly 1 argument: filename.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return LiteralValue::StringValue("File path must be a string".to_string()),
    };

    if fs::metadata(filename).is_ok() {
        LiteralValue::True
    } else {
        LiteralValue::False
    }
}

fn delete_file_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("delete_file requires exactly 1 argument: filename.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return LiteralValue::StringValue("File path must be a string".to_string()),
    };

    match fs::remove_file(filename) {
        Ok(_) => LiteralValue::True,
        Err(e) => LiteralValue::StringValue(format!("Error deleting file: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_package_loading() {
        let parent_env = Rc::new(RefCell::new(Environment::new()));
        let io_env = load_io_package(parent_env).unwrap();
        
        // Test that file functions are available
        assert!(io_env.borrow().get("open_file").is_some());
        assert!(io_env.borrow().get("write_file").is_some());
        assert!(io_env.borrow().get("file_exists").is_some());
        assert!(io_env.borrow().get("delete_file").is_some());
        assert!(io_env.borrow().get("read_input").is_some());
    }
}