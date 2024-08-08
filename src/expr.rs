use crate::scanner::{Token, TokenType};
use crate::scanner;

use LiteralValue::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),            
    StringValue(String),    
    True,                   
    False,                 
    Nil, 
}

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::IntValue(x)) => x as f32,
        Some(scanner::LiteralValue::FloatValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierValue(s)) => s.clone(),
        _ => panic!("Could not unwrap as string"),
    }
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn to_type(&self) -> String {
        match self {
            LiteralValue::Number(_) => "Number".to_string(),
            LiteralValue::StringValue(_) => "String".to_string(),
            LiteralValue::True => "Bool".to_string(),
            LiteralValue::False => "Bool".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::String => Self::StringValue(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token)
        }
    }

    pub fn check_bool(b: bool) -> Self {
        if b {
            True
        } else {
            False
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Number(x) => {
                if *x == 0.0 as f32 {
                    True
                } else {
                    False
                }
            }
            StringValue(s) => {
                if s.len() == 0 {
                    True
                } else {
                    False
                }
            }
            True => False,
            False => True,
            Nil => True,

        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { 
                left, 
                operator, 
                right, 
            } => format!(
                "({} {} {})", 
                operator.lexeme, 
                left.to_string(), 
                right.to_string()
            ),
            Expr::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } => { 
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator_str, right_str)
            }
        }
    }

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Literal { value} => Ok((*value).clone()),
            Expr::Grouping { expression} => expression.evaluate(),
            Expr::Unary { operator, right} => {
                let right = right.evaluate()?;

                match (&right, operator.token_type) {
                    (Number(x), TokenType::Minus) => Ok(Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Cannot use - for {:?}", right.to_type())),
                    
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    (_, ttype) => Err(format!("{} is not a valid operator.", ttype.to_string()))
                }
            }
            Expr::Binary { 
                left, 
                operator, 
                right,
            } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {

                    //PLUS
                    (Number(x), TokenType::Plus, Number(y)) => Ok(Number(x + y)),
                    (StringValue(s1), TokenType::Plus, StringValue(s2)) => { Ok(StringValue(format!("{}{}", s1, s2))) }
                    (StringValue(s1), TokenType::Plus, Number(x)) => Ok(StringValue(format!("{}{}", s1, x.to_string()))),
                    (Number(x), TokenType::Plus, StringValue(s1)) => Ok(StringValue(format!("{}{}", x.to_string(), s1))),

                    (Number(x), TokenType::Minus, Number(y)) => Ok(Number(x - y)),
                    (StringValue(s1), TokenType::Minus, StringValue(s2)) => Err("NaN".to_string()),
                    (StringValue(s1), TokenType::Minus, Number(x)) => Err("NaN".to_string()),
                    (Number(x), TokenType::Minus, StringValue(s1)) => Err("NaN".to_string()),

                    (Number(x), TokenType::Slash, Number(y)) => Ok(Number(x / y)),
                    (Number(x), TokenType::Star, Number(y)) => Ok(Number(x * y)),

                    (Number(x), TokenType::Greater, Number(y)) => Ok(LiteralValue::check_bool(x > y)),
                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 > s2)),
                    (Number(x), TokenType::GreaterEqual, Number(y)) => Ok(LiteralValue::check_bool(x >= y)),
                    (StringValue(s1), TokenType::Greater, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 >= s2)),

                    (Number(x), TokenType::Less, Number(y)) => Ok(LiteralValue::check_bool(x < y)),
                    (StringValue(s1), TokenType::Less, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 < s2)),
                    (Number(x), TokenType::LessEqual, Number(y)) => Ok(LiteralValue::check_bool(x <= y)),
                    (StringValue(s1), TokenType::LessEqual, StringValue(s2)) => Ok(LiteralValue::check_bool(s1 <= s2)),

                    (x, TokenType::BangEqual, y) => Ok(LiteralValue::check_bool(x != y)),
                    (x, TokenType::EqualEqual, y) => Ok(LiteralValue::check_bool(x == y)),
                    (x, ttype, y) => Err(format!("{} has not been implemented", ttype.to_string()))
                }
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::TokenType;

    #[test]
    fn print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };

        let new_number = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };

        let group = Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };

        let multi_token = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let ast = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: minus_token,
                right: Box::new(new_number),
            }),
            operator: multi_token,
            right: Box::new(group),
        };

        // Print the abstract syntax tree.
        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}