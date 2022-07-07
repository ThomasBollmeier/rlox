#[derive(Debug)]
pub enum OpCode {
    Return,
}

impl TryFrom<u8> for OpCode {

    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == OpCode::Return as u8 => Ok(OpCode::Return),
            _ => Err(format!("Unknown opcode {}", value))
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    data: Vec<u8>,
}

impl Chunk {

    pub fn new() -> Chunk {
        Chunk::new_with_capacity(0)
    }
    
    pub fn new_with_capacity(capacity: usize) -> Chunk {
        Chunk {
            data: Vec::with_capacity(capacity),
        }
    }
    
    pub fn write(&mut self, byte: u8) {
        self.data.push(byte);
    }    

    pub fn read(&self, offset: usize) -> Option<u8> {
        match self.data.get(offset) {
            Some(&byte) => Some(byte.clone()),
            None => None
        }
    }
}

