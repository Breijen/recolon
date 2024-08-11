use rand::Rng;

use crate::expr::{Expr, LiteralValue};
use crate::scanner::TokenType;
use crate::parser::Parser;

pub fn check_type(parser: &mut Parser, identifier: String) -> Result<Expr, String>{
    match identifier.as_str() {
        // Constants
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

        //Number representative
        "floor" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'floor'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_floor(arg))
        },
        "ceil" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'ceil'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_ceil(arg))
        },
        "round" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'ceil'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_round(arg))
        },
        "sqrt" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'sqrt'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_sqrt(arg))
        }
        "abs" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'sqrt'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_abs(arg))
        }
        "max" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'max'")?;
            let first_arg = parser.expression()?; // Parse the first argument expression
            parser.consume(TokenType::Comma, "Expected ',' after first argument")?;
            let second_arg = parser.expression()?; // Parse the second argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(get_max(first_arg, second_arg)) // Pass both arguments to get_max
        },
        "min" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'max'")?;
            let first_arg = parser.expression()?; // Parse the first argument expression
            parser.consume(TokenType::Comma, "Expected ',' after first argument")?;
            let second_arg = parser.expression()?; // Parse the second argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(get_min(first_arg, second_arg)) // Pass both arguments to get_max
        },
        "random" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'max'")?;
            let first_arg = parser.expression()?; // Parse the first argument expression
            parser.consume(TokenType::Comma, "Expected ',' after first argument")?;
            let second_arg = parser.expression()?; // Parse the second argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(get_rand(first_arg, second_arg)) // Pass both arguments to get_max
        },

        // Power and logarithmic functions
        "pow" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'max'")?;
            let first_arg = parser.expression()?; // Parse the first argument expression
            parser.consume(TokenType::Comma, "Expected ',' after first argument")?;
            let second_arg = parser.expression()?; // Parse the second argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(get_pow(first_arg, second_arg)) // Pass both arguments to get_max
        },
        "lgm" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'log'")?;
            let first_arg = parser.expression()?; // Parse the first argument expression

            // Check if the next token is a comma to see if there's a second argument
            let second_arg = if parser.check(TokenType::Comma) {
                parser.consume(TokenType::Comma, "Expected ',' after first argument")?;
                Some(parser.expression()?) // Parse the second argument if it exists
            } else {
                None
            };

            parser.consume(TokenType::RightParen, "Expected ')' after arguments")?;

            Ok(get_log(first_arg, second_arg)) // Create log expression with parsed arguments
        },

        // Trigonometric functions
        "cos" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'cos'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_cos(arg))
        },
        "sin" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'sin'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_sin(arg))
        },
        "tan" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'tan'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_tan(arg))
        },

        // Angular conversion
        "degrees" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'tan'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_radians(arg))
        },
        "radians" => {
            parser.consume(TokenType::LeftParen, "Expected '(' after 'tan'")?;
            let arg = parser.expression()?; // Parse the argument expression
            parser.consume(TokenType::RightParen, "Expected ')' after argument")?;

            Ok(get_degrees(arg))
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

pub(crate) fn get_round(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "round".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_sqrt(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "sqrt".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_abs(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "abs".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_max(arg1: Expr, arg2: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "max".to_string(),
        args: vec![arg1, arg2],
    }
}

pub(crate) fn get_min(arg1: Expr, arg2: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "min".to_string(),
        args: vec![arg1, arg2],
    }
}

pub(crate) fn get_rand(arg1: Expr, arg2: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "random".to_string(),
        args: vec![arg1, arg2],
    }
}

pub(crate) fn get_pow(arg1: Expr, arg2: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "pow".to_string(),
        args: vec![arg1, arg2],
    }
}

pub(crate) fn get_log(arg: Expr, base: Option<Expr>) -> Expr {
    let args = match base {
        Some(base_expr) => vec![arg, base_expr],
        None => vec![arg],
    };

    Expr::PreFunction {
        module: "math".to_string(),
        name: "lgm".to_string(),
        args,
    }
}

pub(crate) fn get_cos(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "cos".to_string(),
        args: vec![arg],
    }
}
pub(crate) fn get_sin(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "sin".to_string(),
        args: vec![arg],
    }
}
pub(crate) fn get_tan(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "tan".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_degrees(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "degrees".to_string(),
        args: vec![arg],
    }
}

pub(crate) fn get_radians(arg: Expr) -> Expr {
    Expr::PreFunction {
        module: "math".to_string(),
        name: "radians".to_string(),
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

pub fn round(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("round() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.round())),
        _ => Err("round() requires a numeric argument.".to_string()),
    }
}

pub fn sqrt(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("sqrt() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.sqrt())),
        _ => Err("sqrt() requires a numeric argument.".to_string()),
    }
}

pub fn abs(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("abs() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.abs())),
        _ => Err("abs() requires a numeric argument.".to_string()),
    }
}

pub fn max(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 2 {
        return Err("max() requires two arguments.".to_string());
    }
    match (&args[0], &args[1])  {
        (LiteralValue::Number(a) , LiteralValue::Number(b)) => {
            if a >= b {
                Ok(LiteralValue::Number(*a))
            } else {
                Ok(LiteralValue::Number(*b))
            }
        }
        _ => Err("max() requires two numeric arguments.".to_string()),
    }
}

pub fn min(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 2 {
        return Err("min() requires two arguments.".to_string());
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(a), LiteralValue::Number(b)) => {
            if a <= b {
                Ok(LiteralValue::Number(*a))
            } else {
                Ok(LiteralValue::Number(*b))
            }
        }
        _ => Err("min() requires two numeric arguments.".to_string()),
    }
}

pub fn random(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 2 {
        return Err("random() requires two arguments.".to_string());
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(a), LiteralValue::Number(b)) => {
            if a <= b {
                let num = rand::thread_rng().gen_range(*a..*b);
                Ok(LiteralValue::Number(num.round()))
            } else {
               Err("First argument should be lower than the second argument.".to_string())
            }
        }
        _ => Err("random() requires two numeric arguments.".to_string()),
    }
}

pub fn pow(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 2 {
        return Err("pow() requires two arguments.".to_string());
    }
    match (&args[0], &args[1]) {
        (LiteralValue::Number(a), LiteralValue::Number(b)) => {
            let result = a.powf(*b);
            if result.is_finite() {
                Ok(LiteralValue::Number(result))
            } else {
                Err("Result is not a finite number.".to_string())
            }
        }
        _ => Err("pow() requires two numeric arguments.".to_string()),
    }
}

pub fn lgm(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    match args.len() {
        1 => {
            if let LiteralValue::Number(x) = args[0] {
                if x > 0.0 {
                    let result = x.ln(); // Natural logarithm
                    Ok(LiteralValue::Number(result))
                } else {
                    Err("Logarithm undefined for non-positive values.".to_string())
                }
            } else {
                Err("lgm() requires a numeric argument.".to_string())
            }
        }
        2 => {
            if let (LiteralValue::Number(x), LiteralValue::Number(base)) = (&args[0], &args[1]) {
                if *x > 0.0 && *base > 0.0 && *base != 1.0 {
                    let result = x.log(*base); // Logarithm with specified base
                    Ok(LiteralValue::Number(result))
                } else {
                    Err("Logarithm requires positive x and base != 1.".to_string())
                }
            } else {
                Err("lgm() requires two numeric arguments.".to_string())
            }
        }
        _ => Err("lgm() requires one or two arguments.".to_string()),
    }
}

pub fn cos(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.cos())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}

pub fn sin(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.sin())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}

pub fn tan(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.tan())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}

pub fn degrees(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.to_degrees())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}

pub fn radians(args: Vec<LiteralValue>) -> Result<LiteralValue, String> {
    if args.len() != 1 {
        return Err("ceil() requires exactly one argument.".to_string());
    }
    match args[0] {
        LiteralValue::Number(x) => Ok(LiteralValue::Number(x.to_radians())),
        _ => Err("ceil() requires a numeric argument.".to_string()),
    }
}