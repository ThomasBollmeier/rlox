use std::collections::VecDeque;

use super::{scanner::Scanner, token::{Token, TokenType}};

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    lookahead: VecDeque<Token>,
    previous: Option<Token>,
    current: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl <'a> Compiler<'a> {

    pub fn new(source: &str) -> Compiler {
        Compiler { 
            scanner: Scanner::new(source),
            lookahead: VecDeque::new(),
            previous: None,
            current: None,
            had_error: false, 
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) -> bool {
        self.had_error = false;
        self.panic_mode = false;
        
        self.advance();
        //self.expression();
        self.consume(TokenType::Eof, "expect end of expression.");
        
        !self.had_error
    }

    fn _expression(&mut self) {
        todo!("implement...");
    }

    fn consume(&mut self, expected_type: TokenType, message: &str) {
        if let Some(current) = &self.current { 
            if current.get_token_type() == expected_type {
                self.advance();
                return ;
            }
        }
        self.error_at_current(message);
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            if let Some(next_token) = self.scan_next_token() {
                self.current = Some(next_token);
                if self.current.as_ref().unwrap().get_token_type() != TokenType::Error {
                    break;
                } else {
                    self.error_at_current("a scan error occurred");
                }
            } else {
                self.current = None;
                break;
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current.clone(), message);     
    }

    fn _error(&mut self, message: &str) {
        self.error_at(&self.previous.clone(), message);
    }

    fn error_at(&mut self, token_opt: &Option<Token>, message: &str) {

        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        if let Some(token) = token_opt {
            eprint!("[line {}] Error", token.get_line());
            match token.get_token_type() {
                TokenType::Eof => eprint!(" at end"),
                TokenType::Error => (),
                _ => eprint!(" at '{}'", token.get_lexeme()),
            }
            eprintln!(": {}", message);
        } else {
            eprintln!("Error: {}", message);
        }
        
        self.had_error = true;
    }

    fn scan_next_token(&mut self) -> Option<Token> {
        if self.lookahead.is_empty() {
            self.scanner.next()
        } else {
            Some(self.lookahead.pop_front().unwrap())
        }
    }

    fn _peek(&mut self, idx: usize) -> Option<Token> {
        while idx + 1 > self.lookahead.len() {
            if let Some(token) = self.scanner.next() {
                self.lookahead.push_back(token);
            } else {
                return None;
            }
        }
        Some(self.lookahead[idx].clone())
    }

}