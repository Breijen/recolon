use std::string::String;
use std::collections::HashMap;

use TokenType::*;
use LiteralValue::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Self {
            source: src.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: get_keyword_hashmap(),
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

        if !errors.is_empty() {
            // Join all error messages into a single string, separated by newlines
            let joined = errors.join("\n");
            return Err(joined);
        }

        // Return a clone of the tokens if there are no errors
        Ok(self.tokens.clone())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            ';' => self.add_token(Semicolon),
            ':' => self.add_token(Colon),
            '+' => self.add_token(Plus),
            '-' => self.add_token(Minus),
            '/' => self.add_token(Slash),
            '*' => self.add_token(Star),
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance(); // Skip the rest of the line
                }
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
                    let _ = self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(format!("Unrecognized token '{}' at line {}", c, self.line));
                }
            }
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

        self.add_token_lit(String, Some(StringValue(value.to_string())));

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

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let substring = &self.source[self.start..self.current];
        if let Some(&t_type) = self.keywords.get(substring) {
            self.add_token(t_type)
        } else {
            self.add_token(TokenType::Identifier);
        }
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Plus,
    Minus,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
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
    Function,
    Struct,
    Log,
    Error,
    Print,
    Return,
    Loop,

    Import,
    As,

    Eof,
}

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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
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

//Helper Functions
fn is_digit(ch: char) -> bool {
    ch as u8 >= '0' as u8 && ch as u8 <= '9' as u8
}

fn is_alpha(ch: char) -> bool {
    (ch as u8 >= 'a' as u8 && ch as u8 <= 'z' as u8) || 
    (ch as u8 >= 'A' as u8 && ch as u8 <= 'Z' as u8) ||
    (ch == '_')
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn get_keyword_hashmap() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("if", If), // Works
        ("elif", Elif), // Works
        ("else", Else), // Works
        ("for", For), // Works
        ("in", In), // Works
        ("while", While), // Works
        ("true", True), // Works
        ("false", False), // Works
        ("nil", Nil), // Works
        ("this", This),
        ("and", And), // Works
        ("or", Or), // Works
        ("class", Class),
        ("fn", Function), // Works
        ("struct", Struct), // Works
        ("return", Return), // Works
        ("compose", Loop), // Works
        ("var", Var), // Works
        ("const", Const),
        ("log", Log), // Works
        ("err", Error), // Works
        ("print", Print), // Works
        ("import", Import), // Works
        ("as", As), // Works
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

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
        let _ = scanner.scan_tokens();

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
        let _ = scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(scanner.tokens.len(), 2); 
        assert_eq!(scanner.tokens[0].token_type, String); 
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
        let _ = scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(scanner.tokens.len(), 2); // Expect 2 tokens: StringLit and Eof
        assert_eq!(scanner.tokens[0].token_type, String); // First token should be a StringLit
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
        let _ = scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(scanner.tokens.len(), 4);
        assert_eq!(scanner.tokens[0].token_type, Number); 
        assert_eq!(scanner.tokens[1].token_type, Number); 
        assert_eq!(scanner.tokens[2].token_type, Number); 
        assert_eq!(scanner.tokens[3].token_type, Eof); 
    }

    #[test]
    fn get_identifier() {
        let source = "this_var = 12;";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Identifier); 
        assert_eq!(scanner.tokens[1].token_type, Equal); 
        assert_eq!(scanner.tokens[2].token_type, Number); 
        assert_eq!(scanner.tokens[3].token_type, Semicolon); 
        assert_eq!(scanner.tokens[4].token_type, Eof); 
    }

    #[test]
    fn get_keywords() {
        let source = "var this_var = 12; \nwhile true{ log 3; };";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 14);
        assert_eq!(scanner.tokens[0].token_type, Var); 
        assert_eq!(scanner.tokens[1].token_type, Identifier); 
        assert_eq!(scanner.tokens[2].token_type, Equal); 
        assert_eq!(scanner.tokens[3].token_type, Number); 
        assert_eq!(scanner.tokens[4].token_type, Semicolon); 
        assert_eq!(scanner.tokens[5].token_type, While); 
        assert_eq!(scanner.tokens[6].token_type, True); 
        assert_eq!(scanner.tokens[7].token_type, LeftBrace); 
        assert_eq!(scanner.tokens[8].token_type, Log); 
        assert_eq!(scanner.tokens[9].token_type, Number); 
        assert_eq!(scanner.tokens[10].token_type, Semicolon); 
        assert_eq!(scanner.tokens[11].token_type, RightBrace); 
        assert_eq!(scanner.tokens[12].token_type, Semicolon); 
        assert_eq!(scanner.tokens[13].token_type, Eof); 
    }

    #[test]
    fn handle_single_line_comment() {
        let source = "# This is a comment\nlet x = 10;";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().expect("Failed to scan tokens");

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, Identifier); // Assuming 'let' is an Identifier
        assert_eq!(tokens[1].token_type, Identifier); // Variable name 'x'
        assert_eq!(tokens[2].token_type, Equal);
        assert_eq!(tokens[3].token_type, Number);
        assert_eq!(tokens[4].token_type, Semicolon);
        assert_eq!(tokens[5].token_type, Eof);
    }

}