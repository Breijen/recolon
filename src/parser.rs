use std::string::String;

use crate::scanner::{Token, TokenType, TokenType::*};
use crate::expr::{Expr::*, Expr, LiteralValue};
use crate::stmt::Stmt;

/// Represents the parser structure that processes tokens.
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts= vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errs.push(msg);
                    self.sync();
                },
            }
        }

        if errs.len() == 0 {
            Ok(stmts)
        } else {
            Err(errs.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(Var) {
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(msg) => {
                    Err(msg)
                }
            }
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self.consume(Identifier, "Expected variable name")?;

        let initializer;
        if self.match_token(Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Literal { value: LiteralValue::Nil };
        }

        self.consume(Semicolon, "Expected ';' after variable declaration.")?;

        Ok(Stmt::Var {
            name: token,
            initializer
        })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(Log) {
            self.log_statement()
        } else if self.match_token(Error) {
            self.log_err_statement()
        } else if self.match_token(LeftBrace) {
            self.block_statement()
        } else if self.match_token(If) {
            self.if_statement()
        } else if self.match_token(While) {
            self.while_statement()
        } else if self.match_token(For) {
            self.for_statement()
        } else if self.match_token(Return) {
            self.return_statement()
        } else if self.match_token(Loop) {
            self.loop_statement()
        } else {
            self.expression_statement()
        }
    }

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous(); // 'return' token
        let value = if !self.check(Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(Semicolon, "Expected ';' after return value.")?;
        Ok(Stmt::ReturnStmt { keyword, value })
    }

    fn loop_statement(&mut self) -> Result<Stmt, String> {
        let body = Box::new(self.statement()?);

        Ok(Stmt::LoopStmt { body })
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' after 'if'.")?;
        let predicate = self.expression()?;
        self.consume(RightParen, "Expected ')' after condition.")?;

        let then = Box::new(self.statement()?);

        // Collect elif branches
        let mut elifs = Vec::new();
        while self.match_token(Elif) {  // Assuming Elif is a defined token type
            self.consume(LeftParen, "Expected '(' after 'elif'.")?;
            let elif_predicate = self.expression()?;
            self.consume(RightParen, "Expected ')' after 'elif' condition.")?;
            let elif_body = Box::new(self.statement()?);
            elifs.push((elif_predicate, elif_body));
        }

        // Check for else statement
        let els = if self.match_token(Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::IfStmt {
            predicate,
            then,
            elifs,
            els,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::WhileStmt { condition, body: Box::new(body) })
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' after 'for'.")?;

        // Initialization statement
        let initializer = if self.match_token(Semicolon) {
            None // No initialization
        } else if self.match_token(Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        // Condition expression
        let condition = if self.check(Semicolon) {
            Literal { value: LiteralValue::True } // Default condition is true
        } else {
            self.expression()?
        };
        self.consume(Semicolon, "Expected ';' after loop condition.")?;

        // Increment expression
        let increment = if self.check(RightParen) {
            None // No increment
        } else {
            Some(self.expression()?)
        };
        self.consume(RightParen, "Expected ')' after for clauses.")?;

        // Loop body
        let body = self.statement()?;

        // Desugaring the for-loop into a while-loop
        let mut loop_body = vec![body];
        if let Some(increment) = increment {
            loop_body.push(Stmt::Expression {
                expression: increment
            });
        }

        let loop_body_stmt = Stmt::Block {
            statements: loop_body
        };

        let while_stmt = Stmt::WhileStmt {
            condition,
            body: Box::new(loop_body_stmt)
        };

        let mut block_statements = Vec::new();
        if let Some(init) = initializer {
            block_statements.push(init);
        }

        block_statements.push(while_stmt);

        Ok(Stmt::Block {
            statements: block_statements
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(decl);
        }

        self.consume(RightBrace, "Expected '}' after a block.")?;
        Ok(Stmt::Block { statements })
    }

    fn log_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' before value.")?;
        let value = self.expression()?;
        self.consume(RightParen, "Expected ')' after value.")?;
        self.consume(Semicolon, "Expected ';'.")?;
        Ok(Stmt::Log {
            expression: value
        })
    }

    fn log_err_statement(&mut self) -> Result<Stmt, String> {
        self.consume(LeftParen, "Expected '(' before value.")?;
        let value = self.expression()?;
        self.consume(RightParen, "Expected ')' after value.")?;
        self.consume(Semicolon, "Expected ';'.")?;
        Ok(Stmt::Err {
            expression: value
        })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Expression {
            expression: expr
        })
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_token(Equal) {
            let value = self.assignment()?;

            match expr {
                Variable { name } => {
                    Ok(Assign { name, value: Box::from(value) })
                }
                _ => Err("Invalid assignment target.".to_string())
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(Or) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Logical { left: Box::new(expr), operator, right: Box::new(right), };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(And) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Binary {
                left: Box::new(expr),
                operator, 
                right: Box::new(rhs),
            };
        }

       Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous();
            let rhs = self.factor()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary()?;
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            Ok(Unary {
                operator: op,
                right: Box::from(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.peek();

        let result;
        match token.token_type {
            LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(RightParen, "Expected ')'")?;
                result = Grouping {
                    expression: Box::from(expr),
                };
            }
            False | True | Nil | Number | String => {
                self.advance();
                result = Literal {
                    value: LiteralValue::from_token(token),
                }
            }
            Identifier => {
                self.advance();
                result = Variable {
                    name: self.previous(),
                };
            }

            _ => return Err("Expected expression".to_string())

        }

        Ok(result)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String>{
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            Ok(token)
        } else {
            Err(msg.to_string())
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }

    fn match_token(&mut self, typ: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek().token_type == typ {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }

        false
    }

    /// Advances the parser to the next token and returns the current token.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Returns the current token without advancing.
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Returns the previously parsed token.
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// Checks if the parser has reached the end of the token stream.
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Func | Var | For | If | While | Log | Error | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{Scanner, LiteralValue::*};

    #[test]
    fn test_addition() {
        let four = Token { 
            token_type: Number, 
            lexeme: "4".to_string(), 
            literal: Some(IntValue(4)), 
            line_number: 0 };
        let plus = Token { 
            token_type: Plus, 
            lexeme: "+".to_string(), 
            literal: None, 
            line_number: 0 };
        let three = Token { 
            token_type: Number, 
            lexeme: "3".to_string(), 
            literal: Some(IntValue(3)), 
            line_number: 0 };
        let semicolon = Token { 
            token_type: Semicolon, 
            lexeme: ";".to_string(), 
            literal: None, 
            line_number: 0 };
        let eof = Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: 0 };

        // Vector of tokens to be parsed
        let tokens = vec![four, plus, three, semicolon, eof];

        let mut parser = Parser::new(tokens);

        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(+ 4 3)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_paren() {
        let source = "1 == (3 + 5);";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_expr = parser.parse().unwrap();
        let string_expr = parsed_expr[0].to_string();

        assert_eq!(string_expr, "(== 1 (group (+ 3 5)))");
    }
}