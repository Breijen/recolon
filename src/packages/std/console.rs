use std::cell::RefCell;
use std::rc::Rc;
use colored::Colorize;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

pub fn register_console_functions(env: &Rc<RefCell<Environment>>) {
    // Register color_console function
    env.borrow_mut().define("color_console".to_string(), LiteralValue::Callable {
        name: "color_console".to_string(),
        arity: 3,
        fun: Rc::new(color_console_impl),
    }, true);
}

fn color_console_impl(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() < 3 {
        return LiteralValue::StringValue("color_console function takes three arguments.".to_string());
    }

    let color = match &args[0] {
        LiteralValue::StringValue(s) => s.clone(),
        _ => return LiteralValue::StringValue("First argument must be a text color as a string.".to_string()),
    };

    let bg_color = match &args[1] {
        LiteralValue::StringValue(s) => s.clone(),
        _ => return LiteralValue::StringValue("Second argument must be a background color as a string.".to_string()),
    };

    let text = match &args[2] {
        LiteralValue::StringValue(s) => s.clone(),
        _ => return LiteralValue::StringValue("Third argument must be the text as a string.".to_string()),
    };

    let colored_text = match color.as_str() {
        "red" => text.red(),
        "green" => text.green(),
        "blue" => text.blue(),
        "yellow" => text.yellow(),
        "magenta" => text.magenta(),
        "cyan" => text.cyan(),
        "" => text.white(),
        "black" => text.black(),
        _ => return LiteralValue::StringValue("Unsupported text color.".to_string()),
    };

    let colored_text_with_bg = match bg_color.as_str() {
        "red" => colored_text.on_red().to_string(),
        "green" => colored_text.on_green().to_string(),
        "blue" => colored_text.on_blue().to_string(),
        "yellow" => colored_text.on_yellow().to_string(),
        "magenta" => colored_text.on_magenta().to_string(),
        "cyan" => colored_text.on_cyan().to_string(),
        "white" => colored_text.on_white().to_string(),
        "" => colored_text.on_black().to_string(),
        _ => return LiteralValue::StringValue("Unsupported background color.".to_string()),
    };

    LiteralValue::StringValue(colored_text_with_bg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_console_function() {
        let env = Rc::new(RefCell::new(Environment::new()));
        let args = vec![
            LiteralValue::StringValue("red".to_string()),
            LiteralValue::StringValue("blue".to_string()),
            LiteralValue::StringValue("Hello World".to_string()),
        ];
        let result = color_console_impl(env, &args);
        
        match result {
            LiteralValue::StringValue(colored) => assert!(colored.contains("Hello World")),
            _ => panic!("Expected string result from color_console"),
        }
    }
}