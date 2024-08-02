use std::collections::HashMap;
use crate::token::{Token, TokenType};
use crate::error_list::{BosError, BosErrorList};

pub struct Scanner {
    source_code: Vec<char>,
    start: usize,
    current: usize,
    line: u32,
    line_start: usize,
    had_error: bool,
}


impl Scanner {

    pub fn new(source: String) -> Self {
        Self {
            source_code: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
            had_error: false,
        }
    }


    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, BosErrorList> {
        let mut token_list: Vec<Token> = Vec::new();
        let mut error_list: BosErrorList = BosErrorList::new();
        self.had_error = false;
        
        while !self.is_at_end() {
            let t_type = self.scan_token();
            match t_type {
                Ok(t) => match t {
                    None => {}
                    Some(t) => { token_list.push(self.build_token(t)) }
                },
                Err(e) => { self.had_error = true; error_list.push_error(e); }
            }
            self.start = self.current;
        }
        
        token_list.push(Token::new(TokenType::EOF, String::from(""), self.line, self.get_column()));
        
        return if self.had_error {
            Err(error_list)
        } else {
            Ok(token_list)
        }
    }
    
    fn scan_token(&mut self) -> Result<Option<TokenType>, BosError> {
        let opt_c = self.advance();
        return match opt_c {
            None => { Ok(None) }
            Some(c) => {
                match c {
                    '(' => return Ok(Some(TokenType::LEFT_PAREN)),
                    ')' => return Ok(Some(TokenType::RIGHT_PAREN)),
                    '{' => return Ok(Some(TokenType::LEFT_BRACE)),
                    '}' => return Ok(Some(TokenType::RIGHT_BRACE)),
                    ',' => return Ok(Some(TokenType::COMMA)),
                    '.' => return Ok(Some(TokenType::DOT)),
                    '-' => return Ok(Some(TokenType::MINUS)),
                    '+' => return Ok(Some(TokenType::PLUS)),
                    ';' => return Ok(Some(TokenType::SEMICOLON)),
                    '*' => return Ok(Some(TokenType::STAR)),
                    '%' => return Ok(Some(TokenType::MODULO)),
                    '!' => {
                        return if self.match_current('=') { Ok(Some(TokenType::BANG_EQUAL)) } else { Ok(Some(TokenType::BANG)) }
                    },
                    '=' => {
                        return if self.match_current('=') { Ok(Some(TokenType::EQUAL_EQUAL)) } else { Ok(Some(TokenType::EQUAL)) }
                    },
                    '<' => {
                        return if self.match_current('=') { Ok(Some(TokenType::LESS_EQUAL)) } else { Ok(Some(TokenType::LESS)) }
                    },
                    '>' => {
                        return if self.match_current('=') { Ok(Some(TokenType::GREATER_EQUAL)) } else { Ok(Some(TokenType::GREATER)) }
                    },
                    '/' => {
                        if self.match_current('/') { // Two slashes = comment.
                            while match self.peek() { // Advance to the end of the line.
                                None => { false }
                                Some(c) => { c != '\n' }
                            } {self.advance();}
                        } else if self.match_current('*') { // /* = multiline comment.
                            while match self.peek() {
                                None => { false }
                                Some(c) => {
                                    if c == '\n' {
                                        self.line += 1;
                                        self.line_start = self.current;
                                        true
                                    } else if c == '*' {
                                        self.advance();
                                        !self.match_current('/')
                                    } else {
                                        true
                                    }
                                }
                            } {self.advance();}
                        } else {
                            return Ok(Some(TokenType::SLASH));
                        }
                    },
                    
                    '"' => {
                        return match self.scan_string() {
                            Ok(_) => { Ok(Some(TokenType::STRING_LIT)) }
                            Err(_) => {
                                Err(BosError {
                                    name: "Scan Error".to_string(),
                                    detail: "Failed to parse string.".to_string(),
                                    line: self.line,
                                    column: self.get_column(),
                                })
                            }
                        }
                    },
                    
                    '0'..='9' => {
                        return match self.scan_number() {
                            Ok(t) => { Ok(Some(t)) }
                            Err(_) => {
                                Err(BosError {
                                    name: "Scan Error".to_string(),
                                    detail: "Failed to parse number.".to_string(),
                                    line: self.line,
                                    column: self.get_column(),
                                })
                            }
                        }
                    },
                    
                    'A'..='Z' | 'a'..='z' | '_' => {
                        return match self.scan_identifier() {
                            Ok(t) => { Ok(Some(t)) }
                            Err(_) => {
                                Err(BosError {
                                    name: "Scan Error".to_string(),
                                    detail: "Failed to parse identifier.".to_string(),
                                    line: self.line,
                                    column: self.get_column(),
                                })
                            }
                        }
                    },
                    
                    ' ' => {},
                    '\t' => {},
                    '\r' => {},
                    '\n' => { self.line += 1; self.line_start = self.current; },
                    _ => {
                        return Err(BosError {
                            name: "Scan Error".to_string(),
                            detail: format!("Unexpected character '{}'", c),
                            line: self.line,
                            column: self.get_column(),
                        })
                    }
                }
                Ok(None)
            }
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source_code.len()
    }
    
    fn get_column(&self) -> usize {
        self.start - self.line_start + 1
    }
    
    fn advance(&mut self) -> Option<char> {
        match self.source_code.get(self.current) {
            None => { None }
            Some(c) => { self.current += 1; Some(c.clone()) }
        }
    }
    
    fn peek(&self) -> Option<char> {
        match self.source_code.get(self.current) {
            None => { None }
            Some(c) => { Some(c.clone()) }
        }
    }
    
    fn peek_next(&self) -> Option<char> {
        match self.source_code.get(self.current + 1) {
            None => { None }
            Some(c) => { Some(c.clone()) }
        }
    }
    
    fn match_current(&mut self, c: char) -> bool {
        match self.peek() {
            None => {false}
            Some(p) => {
                if p == c {
                    self.advance();
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Builds and returns an instance of Token based on the lexeme that is currently being looked at.
    fn build_token(&mut self, t_type: TokenType) -> Token {
        let text: String = self.get_lexeme_string();
        
        Token::new(t_type, text, self.line, self.get_column())
    }
    
    /// Return a String containing the text between start and current, ie, the current lexeme being looked at.
    fn get_lexeme_string(&self) -> String {
        match self.source_code.get(self.start..self.current) {
            None => {String::from("")}
            Some(s) => {s.into_iter().collect()}
        }
    }
    
    
    // Parsers for longer lexemes.

    /// Scan a string literal and return the String TokenType.
    fn scan_string(&mut self) -> Result<(),()> {
        while match self.peek() {
            None => { false }
            Some(c) => {
                if c == '"' {
                    self.advance(); // This consumes the closing quotes
                    return Ok(())
                } else if c == '\n' { // Do not allow multiline strings
                    false
                } else {
                    true
                }
            }
        } { self.advance(); };
        
        Err(())
    }

    /// Scan a numeric literal and return a floating-point or integer token as appropriate.
    fn scan_number(&mut self) -> Result<TokenType, ()> {
        let mut has_decimal = false;
        
        while match self.peek() {
            None => { false }
            Some(c) => {
                match c {
                    '0'..='9' => {
                        true
                    },
                    '.' => {
                        if has_decimal { false }
                        else {
                            has_decimal = true;
                            match self.peek_next() {
                                None => { false }
                                Some(c) => { c.is_ascii_digit() }
                            }
                        }
                    }
                    _ => {
                        return if has_decimal { Ok(TokenType::FLOAT_LIT) } else { Ok(TokenType::INTEGER_LIT) };
                    }
                }
            }
        } { self.advance(); };
        
        Err(())
    }
    
    /// Scan an identifier, see whether it matches any reserved word, and return the corresponding TokenType.
    fn scan_identifier(&mut self) -> Result<TokenType, ()> {
        while match self.peek() {
            None => { false }
            Some(c) => {
                Self::is_valid_identifier_char(c)
            }
        } { self.advance(); };
        
        match self.check_reserved_words() {
            None => { Ok(TokenType::IDENTIFIER) }
            Some(t) => { Ok(t) }
        }
    }
    
    /// Check if a given char is a valid character in an identifier. Namely, an alphanumeric or underscore.
    fn is_valid_identifier_char(c: char) -> bool {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => true,
            _ => false
        }
    }
    
    /// Compare the current lexeme to the list of reserved words. If there is a match, return the relevant TokenType.
    fn check_reserved_words(&self) -> Option<TokenType> {
        let word_list = HashMap::from([
            ("and", TokenType::AND),
            ("or", TokenType::OR),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("func", TokenType::FUNC),
            ("for", TokenType::FOR),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ]);
        
        let text: String = self.get_lexeme_string();
        
        match word_list.get(text.to_lowercase().as_str()) {
            None => { None }
            Some(&t) => { Some(t) }
        }
    }
}