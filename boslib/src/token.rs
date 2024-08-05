
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    // Token list not final

    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR, MODULO,

    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,

    IDENTIFIER, CHAR_LIT, STRING_LIT, INTEGER_LIT, FLOAT_LIT, HEX_STRING,

    AND, CLASS, ELSE, FALSE, FUNC, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE, DO,

    EOF,
}



#[derive(Debug)]
pub struct Token {
    t_type: TokenType,
    lexeme: String,

    line: u32,
    column: usize,
}


impl Token {
    
    pub fn new(t_type: TokenType, lexeme: String, line: u32, column: usize) -> Self {
        Token {
            t_type,
            lexeme,
            line,
            column,
        }
    }
    
    pub fn get_type(&self) -> TokenType {
        self.t_type
    }
    
    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }
    
    pub fn get_line_and_column(&self) -> (u32, usize) {
        (self.line, self.column)
    }
    
    pub fn get_line(&self) -> u32 {
        self.line
    }
    
    pub fn get_column(&self) -> usize {
        self.column
    }
}