use std::collections::HashMap;
use crate::expr::{Expr, LiteralValue};
use crate::scanner::TokenType;
use crate::parser::Parser; // Assume the parser struct is imported from somewhere

pub fn check_type(parser: &mut Parser, identifier: String) -> Result<Expr, String>{
    match identifier.as_str() {
        "pi" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_pi()), // Call the function to get PI
        }),
        "e" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_e()), // Call the function to get PI
        }),
        "tau" => Ok(Expr::Literal {
            value: LiteralValue::Number(get_tau()), // Call the function to get PI
        }),
        "nan" => Ok(Expr::Literal {
            value: LiteralValue::Nil, // Call the function to get PI
        }),
        "floor" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'floor'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_floor(arg))
        },
        "ceil" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'floor'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_ceil(arg))
        },
        _ => Err(format!("Unknown identifier '{}'.", identifier)),
    }
}

pub fn get_pi() -> f32 {
    std::f32::consts::PI
}

pub fn get_e() -> f32 {
    std::f32::consts::E
}

pub fn get_tau() -> f32 {
    std::f32::consts::TAU
}

pub(crate) fn get_floor(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "floor".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_ceil(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "ceil".to_string(),
        args: vec![arg],
    }
}

// Define the functions within the module
pub fn floor(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("floor() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.floor())),
        _ => Err("floor() requires a numeric argument.".to_string()),
    }
}

pub fn ceil(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.ceil())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}
