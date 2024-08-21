use std::fs;
use std::io::{self, Write};

use crate::expr::Expr;
use crate::literal_value::LiteralValue;
use crate::parser::Parser;
use crate::scanner::TokenType;


pub fn check_type(parser: &mut Parser, identifier: String) -> Result<Expr, String> {
    match identifier.as_str() {
        "read_input" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'read_input'")?;
            parser.consume(TokenType::RightParen, "Expected ')' after '('")?;

            Ok(fn_read_input())
        },
        "open_file" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'file_open'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(fn_open_file(arg))
        },
        "write_file" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'file_write'")?;
            let filename = parser.expression()?; // Parse the filename argument
            parser.consume(TokenType::Comma, "Expected ',' after filename")?;
            let content = parser.expression()?; // Parse the content argument
            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(fn_write_file(filename, content))
        },
        "file_exists" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'file_exists'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(fn_file_exists(arg))
        },
        "delete_file" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'file_delete'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(fn_delete_file(arg))
        },
        _ => Err(format!("Unknown identifier '{}'.", identifier)),
    }
}

pub(crate) fn fn_read_input() -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "read_input".to_string(),
        args: Vec::new(),
    }
}

pub(crate) fn fn_open_file(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "open_file".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn fn_write_file(filename: Expr, content: Expr) -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "write_file".to_string(),
        args: vec![filename, content],
    }
}

pub(crate) fn fn_file_exists(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "file_exists".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn fn_delete_file(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "delete_file".to_string(),
        args: vec![arg],
    }
}

pub fn read_input() -> Result<LiteralValue, String> {
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before waiting for input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();

    Ok(LiteralValue::StringValue(input))
}

pub fn open_file(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        Err("You can only open one file at a time.".to_string())
    } else {
        let filename = match &args[0] {
            LiteralValue::StringValue(s) => s,
            _ => return Err("File path must be a string".to_string()),
        };

        match fs::read_to_string(filename) {
            Ok(contents) => Ok(LiteralValue::StringValue(contents)),
            Err(e) => Err(format!("Error reading file: {}", e)),
        }
    }
}

pub fn write_file(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 2 {
        return Err("file_write requires exactly 2 arguments: filename and content.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return Err("File path must be a string".to_string()),
    };

    let content = match &args[1] {
        LiteralValue::StringValue(s) => s,
        _ => return Err("File content must be a string".to_string()),
    };

    match fs::write(filename, content) {
        Ok(_) => Ok(LiteralValue::True),
        Err(e) => Err(format!("Error writing to file: {}", e)),
    }
}

pub fn file_exists(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("file_exists requires exactly 1 argument: filename.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return Err("File path must be a string".to_string()),
    };

    if fs::metadata(filename).is_ok() {
        Ok(LiteralValue::True)
    } else {
        Ok(LiteralValue::False)
    }
}

pub fn delete_file(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("file_delete requires exactly 1 argument: filename.".to_string());
    }

    let filename = match &args[0] {
        LiteralValue::StringValue(s) => s,
        _ => return Err("File path must be a string".to_string()),
    };

    match fs::remove_file(filename) {
        Ok(_) => Ok(LiteralValue::True),
        Err(e) => Err(format!("Error deleting file: {}", e)),
    }
}