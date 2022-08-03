use std::fmt::{Display, Debug};

pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Value {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Bool(value) => write!(f, "{}", value),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Debug for Value {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Copy for Value {
    
}

impl Clone for Value {

    fn clone(&self) -> Self {
        match self {
            Self::Number(val) => Self::Number(val.clone()),
            Self::Bool(val) => Self::Bool(val.clone()),
            Self::Nil => Self::Nil,
        }
    }
}