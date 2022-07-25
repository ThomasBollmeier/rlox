use std::{str::Chars, collections::{VecDeque, HashMap}};
use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    source_iter: Chars<'a>,
    lookahead: VecDeque<char>,
    current_lexeme: String,
    current_line: i32,
    keywords: HashMap<String, TokenType>,
}

impl <'a> Scanner<'a> {

    pub fn new(source: &str) -> Scanner {

        let mut keywords = HashMap::new();

        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner {
            source_iter: source.chars(),
            lookahead: VecDeque::new(),
            current_lexeme: String::new(),
            current_line: 1,
            keywords,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.lookahead.is_empty() {
            self.source_iter.next()
        } else {
            let ch = self.lookahead.pop_front().unwrap();
            Some(ch)
        }
    }

    fn peek(&mut self, idx: usize) -> Option<char> {
        while idx + 1 > self.lookahead.len() {
            if let Some(ch) = self.source_iter.next() {
                self.lookahead.push_back(ch);
            } else {
                return None;
            }
        }
        Some(self.lookahead[idx])
    }

    fn scan_token(&mut self, ch: char) -> Token {

        self.current_lexeme.push(ch);

        match ch {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => self.make_one_or_two_char_token('=', 
                TokenType::Bang, TokenType::BangEqual),
            '=' => self.make_one_or_two_char_token('=', 
                TokenType::Equal, TokenType::EqualEqual),
            '<' => self.make_one_or_two_char_token('=', 
                TokenType::Less, TokenType::LessEqual),
            '>' => self.make_one_or_two_char_token('=', 
                TokenType::Greater, TokenType::GreaterEqual),
            '"' => self.scan_string(),
            _ => {
                if ch.is_numeric() {
                    self.scan_number()
                } else if ch.is_alphabetic() || ch == '_' {
                    self.scan_identifier()
                } else {
                    self.make_token(TokenType::Error)
                }
            },
        }
        
    }

    fn scan_identifier(&mut self) -> Token {
        loop {
            if let Some(ch) = self.peek(0) {
                if ch.is_alphanumeric() || ch == '_' {
                    self.current_lexeme.push(ch);
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let lexeme = self.current_lexeme.clone();
        let token_type = self.keywords
            .get(&lexeme)
            .unwrap_or(&TokenType::Identifier);

        Token::new(
            token_type.clone(),
            lexeme,
            self.current_line)
    }

    fn scan_number(&mut self) -> Token {
        loop {
            if let Some(ch) = self.peek(0) {
                if ch.is_numeric() {
                    self.current_lexeme.push(ch);
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Token::new(
            TokenType::Number,
            self.current_lexeme.clone(),
            self.current_line)
    }

    fn scan_string(&mut self) -> Token {
        let start_line = self.current_line;
        loop {
            if let Some(ch) = self.advance() {
                self.current_lexeme.push(ch);
                match ch {
                    '"' => break,
                    '\n' => self.current_line += 1,
                    _ => (), 
                }
            }
            else {
                return Token::new(
                    TokenType::Error,
                    "Unterminated string.".to_string(),
                    start_line);
            }
        }
        Token::new(
            TokenType::String,
            self.current_lexeme.clone(),
            start_line)
    }

    fn make_one_or_two_char_token(&mut self, 
        second_char: char, 
        one_char_type: TokenType,
        two_char_type: TokenType
    ) -> Token {
        if let Some(next_ch) = self.peek(0) {
            if next_ch == second_char {
                let next_char = self.advance().unwrap(); 
                self.current_lexeme.push(next_char);
                self.make_token(two_char_type)
            } else {
                self.make_token(one_char_type)    
            }
        } else {
            self.make_token(one_char_type)
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type, 
            self.current_lexeme.clone(), 
            self.current_line)
    } 

    fn skip_whitespace(&mut self) {
        loop {
            let ch_opt = self.peek(0);
            if ch_opt.is_none() {
                break;
            }
            let ch = ch_opt.unwrap();

            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                },
                '\n' => {
                    self.current_line += 1;
                    self.advance();
                },
                '/' => if let Some(ch2) = self.peek(1) {
                    if ch2 == '/' {
                        self.skip_line_comment();
                    } else {
                        break;
                    }
                } else {
                    break;
                },
                _ => break,
            };
        }
    }

    fn skip_line_comment(&mut self) {
        self.advance();
        self.advance();
        loop {
            if let Some(ch) = self.peek(0) {
                if ch != '\n' {
                    self.advance();
                }
            } else {
                break;
            }
        }

    }

}

impl <'a> Iterator for Scanner<'a> {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        self.skip_whitespace();
        
        if let Some(ch) = self.advance() {
            let token = self.scan_token(ch);
            self.current_lexeme = String::new();
            Some(token)
        } else {
            return None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::frontend::token::Token;
    use crate::frontend::token::TokenType::*;
    use super::Scanner;


    #[test]
    fn scan_simple_statement() {
        let tokens = scan("   if (a == b) { print a + b;} // else ");

        assert!(!tokens.is_empty());
    }

    #[test]
    fn scan_function() {
        let source = "
        fun say_hello(name) {
            print \"Hello \" + name; 
        }
        ";
        
        let tokens: Vec<Token> = scan(source);

        assert!(!tokens.is_empty());

        assert_eq!(
            tokens[0],
            Token::new(Fun, "fun".to_string(), 2)
        );
        assert_eq!(
            tokens[1],
            Token::new(Identifier, "say_hello".to_string(), 2)
        );
        assert_eq!(
            tokens[2],
            Token::new(LeftParen, "(".to_string(), 2)
        );
        assert_eq!(
            tokens[3],
            Token::new(Identifier, "name".to_string(), 2)
        );
        assert_eq!(
            tokens[4],
            Token::new(RightParen, ")".to_string(), 2)
        );
        assert_eq!(
            tokens[5],
            Token::new(LeftBrace, "{".to_string(), 2)
        );
        assert_eq!(
            tokens[6],
            Token::new(Print, "print".to_string(), 3)
        );
        assert_eq!(
            tokens[7],
            Token::new(String, "\"Hello \"".to_string(), 3)
        );
        assert_eq!(
            tokens[8],
            Token::new(Plus, "+".to_string(), 3)
        );
        assert_eq!(
            tokens[9],
            Token::new(Identifier, "name".to_string(), 3)
        );
        assert_eq!(
            tokens[10],
            Token::new(Semicolon, ";".to_string(), 3)
        );
        assert_eq!(
            tokens[11],
            Token::new(RightBrace, "}".to_string(), 4)
        );

    }

    fn scan(source: &str) -> Vec<Token> {
        Scanner::new(source).collect()
    }

}