use std::str::Chars;
use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    source_iter: Chars<'a>,
    lookahead: Option<char>,
    current_lexeme: String,
    current_line: i32,
}

impl <'a> Scanner<'a> {

    pub fn new(source: &str) -> Scanner {

        let mut source_iter = source.chars();
        let lookahead = source_iter.next();

        Scanner {
            source_iter,
            lookahead,
            current_lexeme: String::new(),
            current_line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ret = self.lookahead;
        if ret.is_some() {
            self.lookahead = self.source_iter.next();
        }
        ret
    }

    fn peek(&self) -> &Option<char> {
        &self.lookahead
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
            _ => self.make_token(TokenType::Error),
        }
        
    }

    fn make_one_or_two_char_token(&mut self, 
        second_char: char, 
        one_char_type: TokenType,
        two_char_type: TokenType
    ) -> Token {
        if let Some(next_ch) = self.peek() {
            if *next_ch == second_char {
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

}

impl <'a> Iterator for Scanner<'a> {

    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        
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
    use super::Scanner;


    #[test]
    fn scan_simple_statement() {
        let source = "if (a == b) { print a + b;}";
        let scanner = Scanner::new(&source);
        let tokens: Vec<Token> = scanner.collect();

        assert!(!tokens.is_empty());
    }

}