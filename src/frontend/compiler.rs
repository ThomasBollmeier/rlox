use std::{collections::{VecDeque}, cell::RefCell, rc::Rc};
use crate::backend::{chunk::Chunk, instruction::Instruction, value::Value, heap::HeapManager};
use super::{scanner::Scanner, token::{Token, TokenType}, parse_rules::{Precedence, ParseRules, ParseFn}};

pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    lookahead: VecDeque<Token>,
    previous: Option<Token>,
    current: Option<Token>,
    had_error: bool,
    panic_mode: bool,
    parse_rules: ParseRules,
    heap_manager: Rc<RefCell<HeapManager>>,
}

impl <'a> Compiler<'a> {

    pub fn new(source: &'a str) -> Compiler {
        Self::new_with_heap_mgr(source, &HeapManager::new_rc_refcell())
    }

    pub fn new_with_heap_mgr(source: &'a str, heap_manager: &Rc<RefCell<HeapManager>>) 
        -> Compiler<'a> {

        let mut ret = Compiler { 
            scanner: Scanner::new(source),
            lookahead: VecDeque::new(),
            previous: None,
            current: None,
            had_error: false, 
            panic_mode: false,
            parse_rules: ParseRules::new(),
            heap_manager: heap_manager.clone(),
        };

        ret.init_parse_rules();

        ret
    }

    fn init_parse_rules(&mut self) {

        self.parse_rules.register(
            TokenType::LeftParen,
            grouping(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::Minus,
            unary(),
            binary(),
            Precedence::Term
        );
        self.parse_rules.register(
            TokenType::Plus,
            None,
            binary(),
            Precedence::Term
        );
        self.parse_rules.register(
            TokenType::Slash, 
            None, 
            binary(), 
            Precedence::Factor
        );
        self.parse_rules.register(
            TokenType::Star, 
            None, 
            binary(), 
            Precedence::Factor
        );
        self.parse_rules.register(
            TokenType::Number,
            number(), 
            None, 
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::Nil,
            literal(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::True,
            literal(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::False,
            literal(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::String,
            string(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::Identifier,
            variable(),
            None,
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::Bang, 
            unary(), 
            None, 
            Precedence::None
        );
        self.parse_rules.register(
            TokenType::BangEqual, 
            None, 
            binary(), 
            Precedence::Equality
        );
        self.parse_rules.register(
            TokenType::EqualEqual, 
            None, 
            binary(), 
            Precedence::Equality
        );
        self.parse_rules.register(
            TokenType::Greater, 
            None, 
            binary(), 
            Precedence::Comparison
        );
        self.parse_rules.register(
            TokenType::Greater, 
            None, 
            binary(), 
            Precedence::Comparison
        );
        self.parse_rules.register(
            TokenType::GreaterEqual, 
            None, 
            binary(), 
            Precedence::Comparison
        );
        self.parse_rules.register(
            TokenType::Less, 
            None, 
            binary(), 
            Precedence::Comparison
        );
        self.parse_rules.register(
            TokenType::LessEqual, 
            None, 
            binary(), 
            Precedence::Comparison
        );

    }

    pub fn compile(&mut self, chunk: &mut Chunk) -> bool {
        self.had_error = false;
        self.panic_mode = false;
        
        self.advance();
        
        while !self.is_match(TokenType::Eof) {
            self.declaration(chunk);
        }
        
        self.end_compiler(chunk);
        
        !self.had_error
    }

    fn declaration(&mut self, chunk: &mut Chunk) {

        if self.is_match(TokenType::Var) {
            self.var_declaration(chunk);
        } else {
            self.statement(chunk);
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self, chunk: &mut Chunk) {
        self.consume(TokenType::Identifier, "Expect variable name.");
        let global_idx = self.parse_varname(chunk) as u32;

        if self.is_match(TokenType::Equal) {
            self.expression(chunk);
        } else {
            self.emit_instruction(chunk, Instruction::Nil);
        }

        self.consume(TokenType::Semicolon, 
            "Expect ';' after variable declaration.");

        self.emit_instruction(chunk, Instruction::DefineGlobal { global_idx })
    }

    fn parse_varname(&mut self, chunk: &mut Chunk) -> usize {
        let varname = self.previous.as_ref().unwrap().get_lexeme();
        let varname = HeapManager::malloc(&self.heap_manager, varname.to_string());
        let varname = Value::Str(varname);
        chunk.add_value(varname)
    }

    fn synchronize(&mut self) {

        self.panic_mode = false;

        loop {
            if let Some(token) = &self.current {
                if token.get_token_type() == TokenType::Eof ||
                    self.previous.as_ref().unwrap().get_token_type() == TokenType::Semicolon {
                    return;
                }
                match token.get_token_type() {
                    TokenType::Class |
                    TokenType::Fun |
                    TokenType::Var |
                    TokenType::For |
                    TokenType::If |
                    TokenType::While |
                    TokenType::Print |
                    TokenType::Return =>
                        return,
                    _ => (),
                }


            } else {
                return;
            }

            self.advance();
        }

    }

    fn statement(&mut self, chunk: &mut Chunk) {
        if self.is_match(TokenType::Print) {
            self.print_statement(chunk);
        } else {
            self.expr_statement(chunk);
        }
    }

    fn print_statement(&mut self, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_instruction(chunk, Instruction::Print)
    }

    fn expr_statement(&mut self, chunk: &mut Chunk) {
        self.expression(chunk);
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        self.emit_instruction(chunk, Instruction::Pop);
    }

    fn expression(&mut self, chunk: &mut Chunk) {
        self.parse_precedence(Precedence::Assignment, chunk);
    }

    fn number(&self, chunk: &mut Chunk, _can_assign: bool) {
        if let Some(token) = &self.previous {
            let x = token.get_lexeme().parse::<f64>().unwrap();
            let value = Value::Number(x);
            let value_idx = chunk.add_value(value);
            self.emit_constant(chunk, value_idx);
        } 
    }

    fn literal(&self, chunk: &mut Chunk, _can_assign: bool) {
        if let Some(token) = &self.previous {
            match token.get_token_type() {
                TokenType::Nil => self.emit_instruction(chunk, Instruction::Nil),
                TokenType::True => self.emit_instruction(chunk, Instruction::True),
                TokenType::False => self.emit_instruction(chunk, Instruction::False),
                _ => (),
            }
        }
    }

    fn string(&self, chunk: &mut Chunk, _can_assign: bool) {
        if let Some(token) = &self.previous {
            let s = token.get_lexeme();
            let s = s[1..(s.len()-1)].to_string();
            let s_ref = HeapManager::malloc(&self.heap_manager, s);
            let value = Value::Str(s_ref);
            let value_idx = chunk.add_value(value);
            self.emit_constant(chunk, value_idx);
        }
    }

    fn variable(&mut self, chunk: &mut Chunk, can_assign: bool) {
        if let Some(token) = &self.previous {
            let s = token.get_lexeme().to_string();
            let s_ref = HeapManager::malloc(&self.heap_manager, s);
            let value = Value::Str(s_ref);
            let global_idx = chunk.add_value(value) as u32;

            if !can_assign || !self.is_match(TokenType::Equal) {
                self.emit_instruction(chunk, Instruction::GetGlobal {global_idx});
            } else {
                self.expression(chunk);
                self.emit_instruction(chunk, Instruction::SetGlobal {global_idx});
            }
        }
    }

    fn grouping(&mut self, chunk: &mut Chunk, _can_assign: bool) {
        self.expression(chunk);
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn binary(&mut self, chunk: &mut Chunk, _can_assign: bool) {
        let operator_type = self.previous.as_ref().unwrap().get_token_type();
        let next_prec = self.parse_rules
            .get_parse_rule(&operator_type)
            .precedence.clone()
            .increment();
        
        // parse right hand side
        self.parse_precedence(next_prec, chunk);

        match operator_type {
            TokenType::Plus => self.emit_instruction(chunk, Instruction::Add),
            TokenType::Minus => self.emit_instruction(chunk, Instruction::Subtract),
            TokenType::Star => self.emit_instruction(chunk, Instruction::Multiply),
            TokenType::Slash => self.emit_instruction(chunk, Instruction::Divide),
            TokenType::BangEqual => {
                self.emit_instruction(chunk, Instruction::Equal);
                self.emit_instruction(chunk, Instruction::Not);
            },
            TokenType::EqualEqual => self.emit_instruction(chunk, Instruction::Equal),
            TokenType::Greater => self.emit_instruction(chunk, Instruction::Greater),
            TokenType::GreaterEqual => {
                self.emit_instruction(chunk, Instruction::Less);
                self.emit_instruction(chunk, Instruction::Not);
            },
            TokenType::Less => self.emit_instruction(chunk, Instruction::Less),
            TokenType::LessEqual => {
                self.emit_instruction(chunk, Instruction::Greater);
                self.emit_instruction(chunk, Instruction::Not);
            },
            _ => (),
        }
    }

    fn unary(&mut self, chunk: &mut Chunk, _can_assign: bool) {
        let token_type = &self.previous
            .as_ref()
            .unwrap()
            .get_token_type();

        self.parse_precedence(Precedence::Unary, chunk);

        match token_type {
            TokenType::Minus => self.emit_instruction(chunk, Instruction::Negate),
            TokenType::Bang => self.emit_instruction(chunk, Instruction::Not),
            _ => ()
        }
    } 

    fn parse_precedence(&mut self, prec: Precedence, chunk: &mut Chunk) {
        self.advance();
        let token_type = &self.previous.as_ref().unwrap().get_token_type();
        let prefix_opt = self.parse_rules.get_parse_rule(token_type).prefix;

        if prefix_opt.is_none() {
            self.error("Expect expression.");
            return;
        }

        let prefix = prefix_opt.unwrap();
        let can_assign = prec <= Precedence::Assignment;
        prefix(self, chunk, can_assign);

        while let Some(token) = &self.current {
            let token_type = token.get_token_type();
            let curr_prec = self.parse_rules
                .get_parse_rule(&token_type)
                .precedence;

            if curr_prec < prec {
                break;
            }

            self.advance();

            let infix = self.parse_rules
                .get_parse_rule(&token_type)
                .infix
                .unwrap();

            infix(self, chunk, can_assign);
        }

        if can_assign && self.is_match(TokenType::Equal) {
            self.error("Invalid assignment target.");
        }

    }

    fn end_compiler(&self, chunk: &mut Chunk) {
        self.emit_return(chunk);
    }

    fn emit_return(&self, chunk: &mut Chunk) {
        self.emit_instruction(chunk, Instruction::Return);
    }

    fn emit_constant(&self, chunk: &mut Chunk, value_idx: usize) {
        let instr = if value_idx < 256 {
            Instruction::Constant { value_idx: value_idx as u8 }
        } else {
            Instruction::ConstantLong { value_idx: value_idx as u32 }
        };
        self.emit_instruction(chunk, instr);
    }

    fn emit_instruction(&self, chunk: &mut Chunk, instr: Instruction) {
        let line = if let Some(token) = &self.previous {
            token.get_line()
        } else {
            1
        };
        chunk.write_instruction(instr, line);
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

    fn is_match(&mut self, expected_type: TokenType) -> bool {
        if self.check(expected_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, expected_type: TokenType) -> bool {
        if let Some(current) = &self.current {
            if current.get_token_type() == expected_type {
                true
            } else {
                false
            }
        } else {
            false
        }
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

    fn error(&mut self, message: &str) {
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

fn grouping() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.grouping(chunk, can_assign))
}

fn binary() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.binary(chunk, can_assign))
}

fn unary() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.unary(chunk, can_assign))
}

fn number() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.number(chunk, can_assign))
}

fn literal() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.literal(chunk, can_assign))
}

fn string() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.string(chunk, can_assign))
}

fn variable() -> Option<ParseFn> {
    Some(|comp, chunk, can_assign| comp.variable(chunk, can_assign))
}