#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    _token_type: TokenType,
    _lexeme: String,
    _line: i32,
}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Token {
        Token { 
            _token_type: token_type, 
            _lexeme: lexeme, 
            _line: line 
        }
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
}