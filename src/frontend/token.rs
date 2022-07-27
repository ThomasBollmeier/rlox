#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Token {
        Token { 
            token_type, 
            lexeme, 
            line 
        }
    }

    pub fn get_token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
    // single-character tokens:
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // one or two character tokens:
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals:
    Identifier,
    String,
    Number,
    // Keywords:
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // miscellaneous:
    Error,
    Eof,
}