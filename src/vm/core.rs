use crate::vm::value::Value;

#[derive(Debug)]
pub enum OpCode {
    Constant,
    Return,
}

impl TryFrom<u8> for OpCode {

    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == OpCode::Constant as u8 => Ok(OpCode::Constant),
            v if v == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {}", value))
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    lines: Vec<i32>, // source code line mapping
}

impl Chunk {

    pub fn new() -> Chunk {
        Chunk::new_with_capacity(0)
    }
    
    pub fn new_with_capacity(capacity: usize) -> Chunk {
        Chunk {
            code: Vec::with_capacity(capacity),
            values: Vec::new(),
            lines: Vec::new(),
        }
    }
    
    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }    

    pub fn read(&self, offset: usize) -> Option<u8> {
        match self.code.get(offset) {
            Some(&byte) => Some(byte.clone()),
            None => None
        }
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1    
    }

    pub fn read_value(&self, offset: usize) -> Option<Value> {
        match self.values.get(offset) {
            Some(&value) => Some(value.clone()),
            None => None
        }
    }

    pub fn read_line(&self, offset: usize) -> i32 {
        self.lines[offset]
    }

}

