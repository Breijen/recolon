use std::string::String;

fn is_digit(ch: char) -> bool {
    ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        
        let mut errors = vec![];
    
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            errors.iter().map(|msg| { 
                joined.push_str(&msg);
                joined.push_str("\n");
            });
            return Err(joined)
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '/' => self.add_token(TokenType::Slash),
            '*' => self.add_token(TokenType::Star),
            '#' => {
                loop {
                    if self.peek() == '\n' || self.is_at_end() {
                        break;
                    }
                    self.advance();
                }
                self.add_token(TokenType::Comment);
            },
            '!' => {
                let token = if self.char_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            },
            '=' => {
                let token = if self.char_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            },
            '<' => {
                let token = if self.char_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            },
            '>' => {
                let token = if self.char_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string()?,
            c => {
                if is_digit(c) {
                    self.number();
                } else {
                    return Err(format!("Unrecognized token '{}' at line {}", c, self.line));
                }
            }
            _ => return Err(format!("Unrecognized token '{}' at line {}", c, self.line)),
        }

        Ok(())
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn char_match(&mut self, _ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != _ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("String not closed.".to_string())
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1]; 

        self.add_token_lit(StringLit, Some(StringValue(value.to_string())));

        Ok(())
    }

    fn number(&mut self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let substring = &self.source[self.start..self.current];
        let value = substring.parse::<f64>();
        match value {
            Ok(value) => self.add_token_lit(Number, Some(FloatValue(value))),
            Err(_) => return Err(format!("Could not parse number: {}", substring))
        } 

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        c 
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: None,
            line_number: self.line,
        });
    }

    fn add_token_lit(
        &mut self,
        token_type: TokenType,
        literal: Option<LiteralValue>
    ) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Plus,
    Minus,
    Slash,
    Star,
    Comment,
    Whitespace,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    StringLit,
    Number,
    Var,
    Const,

    If,
    Elif,
    Else,
    For,
    In,
    While,
    True,
    False,
    Nil,
    This,
    And,
    Or,
    Class,
    Return,

    Eof,
}

use TokenType::*;

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}
use LiteralValue::*;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_lit() {
        let source = r#""Hallo Breijen""#; // Include quotes in the string literal
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(scanner.tokens.len(), 2); // Expect 2 tokens: StringLit and Eof
        assert_eq!(scanner.tokens[0].token_type, StringLit); // First token should be a StringLit
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "Hallo Breijen"),
            _ => panic!("Incorrect literal type"),
        }
        assert_eq!(scanner.tokens[1].token_type, Eof); // Second token should be Eof
    }

    #[test]
    fn handle_string_lit_unterminated() {
        let source = r#""Hallo Breijen"#; // Include quotes in the string literal
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        match result {
            Err(_) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn handle_string_lit_multiline() {
        let source = "\"Hallo\ndef\""; 
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(scanner.tokens.len(), 2); // Expect 2 tokens: StringLit and Eof
        assert_eq!(scanner.tokens[0].token_type, StringLit); // First token should be a StringLit
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "Hallo\ndef"),
            _ => panic!("Incorrect literal type"),
        }
        assert_eq!(scanner.tokens[1].token_type, Eof); // Second token should be Eof
    }

    #[test]
    fn number_literals() {
        let source = "123.123\n321.0\n5"; 
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().expect("Failed to scan tokens");

        println!("{:?}", scanner.tokens);
        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].token_type, Number); 
        assert_eq!(scanner.tokens[1].token_type, Number); 
        assert_eq!(scanner.tokens[2].token_type, Number); 
        assert_eq!(scanner.tokens[3].token_type, Eof); 
    }

}