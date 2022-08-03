use std::fmt::Display;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    ConstantLong,
    Nil,
    True,
    False,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

impl TryFrom<u8> for OpCode {

    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == OpCode::Constant as u8 => Ok(OpCode::Constant),
            v if v == OpCode::ConstantLong as u8 => Ok(OpCode::ConstantLong),
            v if v == OpCode::Nil as u8 => Ok(OpCode::Nil),
            v if v == OpCode::True as u8 => Ok(OpCode::True),
            v if v == OpCode::False as u8 => Ok(OpCode::False),
            v if v == OpCode::Negate as u8 => Ok(OpCode::Negate),
            v if v == OpCode::Add as u8 => Ok(OpCode::Add),
            v if v == OpCode::Subtract as u8 => Ok(OpCode::Subtract),
            v if v == OpCode::Multiply as u8 => Ok(OpCode::Multiply),
            v if v == OpCode::Divide as u8 => Ok(OpCode::Divide),
            v if v == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {}", value))
        }
    }
}

pub enum Instruction {
    Constant{value_idx: u8},
    ConstantLong{value_idx: u32},
    Nil, 
    True,
    False,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

impl Display for Instruction {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant { value_idx } => write!(f, "Constant({})", 
            value_idx),
            Self::ConstantLong { value_idx } => write!(f, "ConstantLong({})", 
            value_idx),
            Self::Nil => write!(f, "Nil"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Negate => write!(f, "Negate"),
            Self::Add => write!(f, "Add"),
            Self::Subtract => write!(f, "Subtract"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Return => write!(f, "Return"), 
        }
    }
}
