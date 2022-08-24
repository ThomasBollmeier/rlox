use std::collections::HashMap;

use crate::backend::chunk::Chunk;

use super::{token::TokenType, compiler::Compiler};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call, 
    Primary,
}

impl Precedence {

    pub fn increment(&self) -> Precedence {
        if *self != Precedence::Primary {
            let value = *self as u8;
            Precedence::try_from(value + 1).unwrap()
        } else {
            self.clone()
        }
    }
    
}

impl TryFrom<u8> for Precedence {
    
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Precedence::None as u8 => Ok(Precedence::None),
            v if v == Precedence::Assignment as u8 => Ok(Precedence::Assignment),
            v if v == Precedence::Or as u8 => Ok(Precedence::Or),
            v if v == Precedence::And as u8 => Ok(Precedence::And),
            v if v == Precedence::Equality as u8 => Ok(Precedence::Equality),
            v if v == Precedence::Comparison as u8 => Ok(Precedence::Comparison),
            v if v == Precedence::Term as u8 => Ok(Precedence::Term),
            v if v == Precedence::Factor as u8 => Ok(Precedence::Factor),
            v if v == Precedence::Unary as u8 => Ok(Precedence::Unary),
            v if v == Precedence::Call as u8 => Ok(Precedence::Call),
            v if v == Precedence::Primary as u8 => Ok(Precedence::Primary),
            _ => Err(format!("Unknown precedence {}", value))
        }
    }
}

pub type ParseFn = fn (&mut Compiler, chunk: &mut Chunk, can_assign: bool) -> ();

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}


pub struct ParseRules {
    rules: HashMap<TokenType, ParseRule>,
}

impl ParseRules {

    pub fn new() -> ParseRules {
        ParseRules { rules: HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn register(&mut self,
        token_type: TokenType, 
        prefix: Option<ParseFn>,
        infix: Option<ParseFn>,
        precedence: Precedence 
    ) {
        self.rules.insert(
            token_type, 
            ParseRule { prefix, infix, precedence }
        );
    }

    pub fn get_parse_rule(&self, token_type: &TokenType) -> &ParseRule {
        self.rules.get(token_type).unwrap_or(
            &ParseRule{
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Precedence;
    use super::Precedence::*;

    #[test]
    fn prec_increment() {
        assert_prec_increment(None, Assignment);
        assert_prec_increment(Assignment, Or);
        assert_prec_increment(Or, And);
        assert_prec_increment(And, Equality);
        assert_prec_increment(Equality, Comparison);
        assert_prec_increment(Comparison, Term);
        assert_prec_increment(Term, Factor);
        assert_prec_increment(Factor, Unary);
        assert_prec_increment(Unary, Call);
        assert_prec_increment(Call, Primary);
        assert_prec_increment(Primary, Primary);

    }

    fn assert_prec_increment(prec: Precedence, exp: Precedence) {
        assert_eq!(prec.increment(), exp);
    }

}