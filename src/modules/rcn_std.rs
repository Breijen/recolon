use std::cell::RefCell;
use std::rc::Rc;
use crate::environment::Environment;
use crate::literal_value::LiteralValue;

use colored::Colorize;

pub(crate) fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValue>) -> LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValue::Number(now as f32 / 1000.0)
}

pub fn color_text(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
    if args.len() < 2 {
        return LiteralValue::StringValue("color_text function takes two arguments.".to_string());
    }

    let color = match &args[0] {
        LiteralValue::StringValue(s) => s.clone(),
        _ => return LiteralValue::StringValue("Second argument must be a color as a string.".to_string()),
    };

    let text = match &args[1] {
        LiteralValue::StringValue(s) => s.clone(),
        _ => return LiteralValue::StringValue("First argument must be a string.".to_string()),
    };

    let colored_text = match color.as_str() {
        "red" => text.red().to_string(),
        "green" => text.green().to_string(),
        "blue" => text.blue().to_string(),
        "yellow" => text.yellow().to_string(),
        "magenta" => text.magenta().to_string(),
        "cyan" => text.cyan().to_string(),
        "white" => text.white().to_string(),
        "black" => text.black().to_string(),
        _ => return LiteralValue::StringValue("Unsupported color.".to_string()),
    };

    LiteralValue::StringValue(colored_text)
}