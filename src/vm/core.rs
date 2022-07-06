#[derive(Debug)]
pub enum OpCode {
    Return,
}

#[derive(Debug)]
pub struct Chunk {
    data: Vec<u8>,
}

impl Chunk {
    
    pub fn new(capacity: usize) -> Chunk {
        Chunk {
            data: Vec::with_capacity(capacity),
        }
    }
    
    pub fn write(&mut self, byte: u8) {
        self.data.push(byte);
    }    
}

