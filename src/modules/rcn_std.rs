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

pub fn color_console(_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValue>) -> LiteralValue {
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
