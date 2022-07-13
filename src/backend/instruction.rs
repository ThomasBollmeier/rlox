use std::fmt::Display;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    ConstantLong,
    Return,
}

impl TryFrom<u8> for OpCode {

    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == OpCode::Constant as u8 => Ok(OpCode::Constant),
            v if v == OpCode::ConstantLong as u8 => Ok(OpCode::ConstantLong),
            v if v == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {}", value))
        }
    }
}

pub enum Instruction {
    Constant{value_idx: u8},
    ConstantLong{value_idx: u32},
    Return,
}

impl Display for Instruction {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant { value_idx } => write!(f, "Constant({})", 
            value_idx),
            Self::ConstantLong { value_idx } => write!(f, "ConstantLong({})", 
            value_idx),
            Self::Return => write!(f, "Return"), 
        }
    }
}
