use crate::scanner::{Token, TokenType};

pub enum LiteralValue {
    Number(f32),            
    StringValue(String),    
    True,                   
    False,                 
    Nil, 
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
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expr> },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string())
            },
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string())
            },
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            },
            Expr::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            },
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