use std::fmt::Display;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    ConstantLong,
    Nil,
    True,
    False,
    Negate,
    Not,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Print,
    Pop,
    DefineGlobal,
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
            v if v == OpCode::Not as u8 => Ok(OpCode::Not),
            v if v == OpCode::Equal as u8 => Ok(OpCode::Equal),
            v if v == OpCode::Greater as u8 => Ok(OpCode::Greater),
            v if v == OpCode::Less as u8 => Ok(OpCode::Less),
            v if v == OpCode::Add as u8 => Ok(OpCode::Add),
            v if v == OpCode::Subtract as u8 => Ok(OpCode::Subtract),
            v if v == OpCode::Multiply as u8 => Ok(OpCode::Multiply),
            v if v == OpCode::Divide as u8 => Ok(OpCode::Divide),
            v if v == OpCode::Return as u8 => Ok(OpCode::Return),
            v if v == OpCode::Print as u8 => Ok(OpCode::Print),
            v if v == OpCode::Pop as u8 => Ok(OpCode::Pop),
            v if v == OpCode::DefineGlobal as u8 => Ok(OpCode::DefineGlobal),
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
    Not,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
    Print,
    Pop,
    DefineGlobal{global_idx: u32},
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
            Self::Not => write!(f, "Not"),
            Self::Equal => write!(f, "Equal"),
            Self::Greater => write!(f, "Greater"),
            Self::Less => write!(f, "Less"),
            Self::Add => write!(f, "Add"),
            Self::Subtract => write!(f, "Subtract"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
            Self::Return => write!(f, "Return"), 
            Self::Print => write!(f, "Print"),
            Self::Pop => write!(f, "Pop"),
            Self::DefineGlobal{global_idx} => 
                write!(f, "DefineGlobal({})", global_idx),
        }
    }
}
