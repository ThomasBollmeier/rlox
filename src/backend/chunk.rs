use super::{value::Value, instruction::{Instruction, OpCode}};

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    lines: Vec<(i32, usize)>, // source code line mapping
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

        if !self.lines.is_empty() {
            let previous = self.lines.last_mut().unwrap();
            if previous.0 == line {
                previous.1 += 1;
            } else {
                self.lines.push((line, 1));
            }
        } else {
            self.lines.push((line, 1));
        }
    }
    
    pub fn write_long(&mut self, intval: u32, line: i32) {
        let bytes = intval.to_be_bytes();
        bytes.into_iter().for_each(|byte| { 
            self.write(byte, line);
        });
    }

    pub fn read(&self, offset: usize) -> Option<&u8> {
        self.code.get(offset)
    }

    pub fn read_n_bytes(&self, offset: usize, n: usize) -> &[u8] {
        &self.code[offset..(offset + n)]
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let bytes_slice = self.read_n_bytes(offset, 4);
        let mut bytes: [u8; 4] = [0; 4];
        for (i, byte) in bytes_slice.iter().enumerate() {
            bytes[i] = *byte;
        }
        u32::from_be_bytes(bytes) 
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1    
    }

    pub fn read_value(&self, offset: usize) -> Option<&Value> {
        self.values.get(offset)
    }

    pub fn get_line(&self, offset: usize) -> Option<i32> {
        let mut total_offset = 0;

        for (line, cnt) in self.lines.iter() {
            total_offset += cnt;
            if offset < total_offset {
                return Some(line.clone());
            }
        }

        None
    }

    pub fn instruction_iter(&self) -> InstructionIter {
        InstructionIter { 
            chunk: self, 
            offset: 0 }
    } 

}

pub struct InstructionIter<'a> {
    chunk: &'a Chunk,
    offset: usize
}

impl <'a> Iterator for InstructionIter<'a> {
    
    type Item = (Instruction, usize);

    fn next(&mut self) -> Option<Self::Item> {
        
        let offset = self.offset;
        
        let op_code_opt = self.chunk.read(self.offset);
        self.offset += 1;

        if op_code_opt.is_none() {
            return None;
        }

        let op_code = op_code_opt.unwrap().clone();
        let op_code_res: Result<OpCode, String> = op_code.try_into();

        if op_code_res.is_err() {
            return None;
        }

        match op_code_res.unwrap() {
            OpCode::Constant => {
                let value_idx_opt = self.chunk.read(self.offset);
                self.offset += 1;
                match value_idx_opt {
                    Some(value_idx) => 
                    Some((Instruction::Constant { value_idx: value_idx.clone() }, offset)),
                    None => None
                } 
            },
            OpCode::ConstantLong => {
                let value_idx = self.chunk.read_u32(self.offset);
                self.offset += 4;
                Some((Instruction::ConstantLong { value_idx: value_idx.clone() }, 
                    offset))
            },
            OpCode::Return => {
                Some((Instruction::Return, offset))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::instruction::OpCode;
    use crate::backend::chunk::Chunk;
    
    #[test]
    fn iterate_chunk() {
     
        let mut chunk = Chunk::new();
    
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(42, 1);
        chunk.write(OpCode::Constant as u8, 2);
        chunk.write(23, 2);
        chunk.write(OpCode::Return as u8, 2);
        chunk.write(OpCode::ConstantLong as u8, 3);
        chunk.write_long(625, 3);
        
        
        for (instr, offset) in chunk.instruction_iter() {
            println!("{:04} {}", offset, instr);
        }
    }

}