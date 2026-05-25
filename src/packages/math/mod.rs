use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

pub fn load_math_package(parent_env: Rc<RefCell<Environment>>) -> Result<Rc<RefCell<Environment>>, String> {
    let math_env = Rc::new(RefCell::new(Environment::new_with_enclosing(parent_env)));

    // Register mathematical constants
    math_env.borrow_mut().define("pi".to_string(), LiteralValue::Number(std::f64::consts::PI), true);
    math_env.borrow_mut().define("e".to_string(), LiteralValue::Number(std::f64::consts::E), true);
    math_env.borrow_mut().define("tau".to_string(), LiteralValue::Number(std::f64::consts::TAU), true);
    math_env.borrow_mut().define("nan".to_string(), LiteralValue::Nil, true);

    // Register mathematical functions
    math_env.borrow_mut().define("floor".to_string(), LiteralValue::Callable {
        name: "floor".to_string(),
        arity: 1,
        fun: Rc::new(floor_impl),
    }, true);

    math_env.borrow_mut().define("ceil".to_string(), LiteralValue::Callable {
        name: "ceil".to_string(),
        arity: 1,
        fun: Rc::new(ceil_impl),
    }, true);

    math_env.borrow_mut().define("round".to_string(), LiteralValue::Callable {
        name: "round".to_string(),
        arity: 1,
        fun: Rc::new(round_impl),
    }, true);

    math_env.borrow_mut().define("sqrt".to_string(), LiteralValue::Callable {
        name: "sqrt".to_string(),
        arity: 1,
        fun: Rc::new(sqrt_impl),
    }, true);

    math_env.borrow_mut().define("abs".to_string(), LiteralValue::Callable {
        name: "abs".to_string(),
        arity: 1,
        fun: Rc::new(abs_impl),
    }, true);

    math_env.borrow_mut().define("pow".to_string(), LiteralValue::Callable {
        name: "pow".to_string(),
        arity: 2,
        fun: Rc::new(pow_impl),
    }, true);

    math_env.borrow_mut().define("min".to_string(), LiteralValue::Callable {
        name: "min".to_string(),
        arity: 2,
        fun: Rc::new(min_impl),
    }, true);

    math_env.borrow_mut().define("max".to_string(), LiteralValue::Callable {
        name: "max".to_string(),
        arity: 2,
        fun: Rc::new(max_impl),
    }, true);

    math_env.borrow_mut().define("random".to_string(), LiteralValue::Callable {
        name: "random".to_string(),
        arity: 0,
        fun: Rc::new(random_impl),
    }, true);

    math_env.borrow_mut().define("random_range".to_string(), LiteralValue::Callable {
        name: "random_range".to_string(),
        arity: 2,
        fun: Rc::new(random_range_impl),
    }, true);

    math_env.borrow_mut().define("mod".to_string(), LiteralValue::Callable {
        name: "mod".to_string(),
        arity: 2,
        fun: Rc::new(mod_impl),
    }, true);

    Ok(math_env)
}

fn floor_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("floor requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.floor()),
        _ => LiteralValue::StringValue("floor requires a number argument.".to_string()),
    }
}

fn ceil_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("ceil requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.ceil()),
        _ => LiteralValue::StringValue("ceil requires a number argument.".to_string()),
    }
}

fn round_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("round requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.round()),
        _ => LiteralValue::StringValue("round requires a number argument.".to_string()),
    }
}

fn sqrt_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("sqrt requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.sqrt()),
        _ => LiteralValue::StringValue("sqrt requires a number argument.".to_string()),
    }
}

fn abs_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("abs requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(n) => LiteralValue::Number(n.abs()),
        _ => LiteralValue::StringValue("abs requires a number argument.".to_string()),
    }
}

fn pow_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("pow requires exactly two arguments.".to_string());
    }

    match (&args[0], &args[1]) {
        (LiteralValue::Number(base), LiteralValue::Number(exp)) => LiteralValue::Number(base.powf(*exp)),
        _ => LiteralValue::StringValue("pow requires number arguments.".to_string()),
    }
}

fn min_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("min requires exactly two arguments.".to_string());
    }

    match (&args[0], &args[1]) {
        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a.min(*b)),
        _ => LiteralValue::StringValue("min requires number arguments.".to_string()),
    }
}

fn max_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("max requires exactly two arguments.".to_string());
    }

    match (&args[0], &args[1]) {
        (LiteralValue::Number(a), LiteralValue::Number(b)) => LiteralValue::Number(a.max(*b)),
        _ => LiteralValue::StringValue("max requires number arguments.".to_string()),
    }
}

fn random_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let mut rng = rand::thread_rng();
    LiteralValue::Number(rng.random::<f64>())
}

fn random_range_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("random_range requires exactly two arguments.".to_string());
    }

    match (&args[0], &args[1]) {
        (LiteralValue::Number(min), LiteralValue::Number(max)) => {
            let mut rng = rand::thread_rng();
            LiteralValue::Number(rng.gen_range(*min..*max))
        },
        _ => LiteralValue::StringValue("random_range requires number arguments.".to_string()),
    }
}

fn mod_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 2 {
        return LiteralValue::StringValue("mod requires exactly two arguments.".to_string());
    }

    match (&args[0], &args[1]) {
        (LiteralValue::Number(x), LiteralValue::Number(y)) => {
            if *y == 0.0 {
                LiteralValue::StringValue("mod by zero is not allowed.".to_string())
            } else {
                LiteralValue::Number(x % y)
            }
        },
        _ => LiteralValue::StringValue("mod requires number arguments.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_package_loading() {
        let parent_env = Rc::new(RefCell::new(Environment::new()));
        let math_env = load_math_package(parent_env).unwrap();
        
        // Test that constants are available
        assert!(math_env.borrow().get("pi").is_some());
        assert!(math_env.borrow().get("e").is_some());
        
        // Test that functions are available
        assert!(math_env.borrow().get("floor").is_some());
        assert!(math_env.borrow().get("ceil").is_some());
        assert!(math_env.borrow().get("sqrt").is_some());
        assert!(math_env.borrow().get("mod").is_some());
    }

    #[test]
    fn test_floor_function() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let args = vec![LiteralValue::Number(3.7)];
        let result = floor_impl(env, &args);
        
        match result {
            LiteralValue::Number(n) => assert_eq!(n, 3.0),
            _ => panic!("Expected number result from floor"),
        }
    }

    #[test]
    fn test_mod_function() {
        let env = Rc::new(RefCell::new(Environment::new()));
        
        // Test basic modulo operation
        let args = vec![LiteralValue::Number(10.0), LiteralValue::Number(3.0)];
        let result = mod_impl(env.clone(), &args);
        match result {
            LiteralValue::Number(n) => assert_eq!(n, 1.0),
            _ => panic!("Expected number result from mod"),
        }
        
        // Test modulo with negative numbers
        let args = vec![LiteralValue::Number(-10.0), LiteralValue::Number(3.0)];
        let result = mod_impl(env.clone(), &args);
        match result {
            LiteralValue::Number(n) => assert_eq!(n, -1.0),
            _ => panic!("Expected number result from mod"),
        }
        
        // Test modulo by zero (should return error)
        let args = vec![LiteralValue::Number(10.0), LiteralValue::Number(0.0)];
        let result = mod_impl(env.clone(), &args);
        match result {
            LiteralValue::StringValue(msg) => assert!(msg.contains("mod by zero")),
            _ => panic!("Expected error message for mod by zero"),
        }
        
        // Test with wrong number of arguments
        let args = vec![LiteralValue::Number(10.0)];
        let result = mod_impl(env.clone(), &args);
        match result {
            LiteralValue::StringValue(msg) => assert!(msg.contains("exactly two arguments")),
            _ => panic!("Expected error message for wrong argument count"),
        }
    }
}