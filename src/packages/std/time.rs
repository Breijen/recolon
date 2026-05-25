use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

pub fn register_time_functions(env: &Rc<RefCell<Environment>>) {
    // Register clock function
    env.borrow_mut().define("clock".to_string(), LiteralValue::Callable {
        name: "clock".to_string(),
        arity: 0,
        fun: Rc::new(clock_impl),
    }, true);

    // Register wait_ms function
    env.borrow_mut().define("wait".to_string(), LiteralValue::Callable {
        name: "wait".to_string(),
        arity: 1,
        fun: Rc::new(wait_ms_impl),
    }, true);
}

fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f64 / 1000.0)
}

fn wait_ms_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() != 1 {
        return LiteralValue::StringValue("wait_ms function requires exactly one argument.".to_string());
    }

    match &args[0] {
        LiteralValue::Number(ms) => {
            let duration = Duration::from_millis(*ms as u64);
            sleep(duration);
            LiteralValue::Nil
        },
        _ => LiteralValue::StringValue("wait_ms function requires a number as the argument.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_function() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let args = vec![];
        let result = clock_impl(env, &args);
        
        match result {
            LiteralValue::Number(time) => assert!(time > 0.0),
            _ => panic!("Expected number result from clock"),
        }
    }

    #[test]
    fn test_wait_ms_function() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let args = vec![LiteralValue::Number(1.0)];
        let result = wait_ms_impl(env, &args);
        
        assert_eq!(result, LiteralValue::Nil);
    }
}