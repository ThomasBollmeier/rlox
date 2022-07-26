use std::collections::VecDeque;

use super::{scanner::Scanner, token::Token};

pub struct Compiler<'a> {
    _scanner: Scanner<'a>,
    _lookahead: VecDeque<Token>,
}

impl <'a> Compiler<'a> {

    pub fn new(source: &str) -> Compiler {
        Compiler { 
            _scanner: Scanner::new(source),
            _lookahead: VecDeque::new(), 
        }
    }

    fn _advance(&mut self) -> Option<Token> {
        if self._lookahead.is_empty() {
            self._scanner.next()
        } else {
            Some(self._lookahead.pop_front().unwrap())
        }
    }

    fn _peek(&mut self, idx: usize) -> Option<Token> {
        while idx + 1 > self._lookahead.len() {
            if let Some(token) = self._scanner.next() {
                self._lookahead.push_back(token);
            } else {
                return None;
            }
        }
        Some(self._lookahead[idx].clone())
    }

}