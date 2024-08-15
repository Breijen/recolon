use std::fs;
use std::io::{self, Write}; // Import necessary modules for I/O

use crate::expr::Expr;
use crate::literal_value::LiteralValue;
use crate::parser::Parser;
use crate::scanner::TokenType;


pub fn check_type(parser: &mut Parser, identifier: String) -> Result<Expr, String> {
    match identifier.as_str() {
        "read_input" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'read_input'")?;
            parser.consume(TokenType::RightParen, "Expected ')' after '('")?;

            Ok(get_input())
        },
        "file_open" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'file_open'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_file(arg))
        },
        _ => Err(format!("Unknown identifier '{}'.", identifier)),
    }
}

pub(crate) fn get_input() -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "read_input".to_string(),
        args: Vec::new(),
    }
}

pub(crate) fn get_file(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "io".to_string(),
        name: "file_open".to_string(),
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