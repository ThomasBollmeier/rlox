use std::collections::HashMap;

use super::{value::Value, instruction::{Instruction, OpCode}};

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    string_idxs: HashMap<String, usize>,
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
            string_idxs: HashMap::new(),
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
    
    pub fn write_u16(&mut self, intval: u16, line: i32) {
        let bytes = intval.to_be_bytes();
        bytes.into_iter().for_each(|byte| { 
            self.write(byte, line);
        });
    }

    pub fn write_long(&mut self, intval: u32, line: i32) {
        let bytes = intval.to_be_bytes();
        bytes.into_iter().for_each(|byte| { 
            self.write(byte, line);
        });
    }

    pub fn write_instruction(&mut self, instr: Instruction, line: i32) -> (usize, usize) {

        let start_offset = self.code.len();

        match instr { 
            Instruction::Constant { value_idx } => {
                self.write(OpCode::Constant as u8, line);
                self.write(value_idx, line);
            },
            Instruction::ConstantLong { value_idx } => {
                self.write(OpCode::ConstantLong as u8, line);
                self.write_long(value_idx, line);
            },
            Instruction::DefineGlobal { global_idx } => {
                self.write(OpCode::DefineGlobal as u8, line);
                self.write_long(global_idx, line);
            },
            Instruction::GetGlobal { global_idx } => {
                self.write(OpCode::GetGlobal as u8, line);
                self.write_long(global_idx, line);
            },
            Instruction::SetGlobal { global_idx } => {
                self.write(OpCode::SetGlobal as u8, line);
                self.write_long(global_idx, line);
            },
            Instruction::GetLocal { local_idx } => {
                self.write(OpCode::GetLocal as u8, line);
                self.write_long(local_idx, line);
            },
            Instruction::SetLocal { local_idx } => {
                self.write(OpCode::SetLocal as u8, line);
                self.write_long(local_idx, line);
            },
            Instruction::Nil =>
                self.write(OpCode::Nil as u8, line),
            Instruction::True =>
                self.write(OpCode::True as u8, line),
            Instruction::False =>
                self.write(OpCode::False as u8, line),
            Instruction::Negate =>
                self.write(OpCode::Negate as u8, line),
            Instruction::Not =>
                self.write(OpCode::Not as u8, line),
            Instruction::Equal =>
                self.write(OpCode::Equal as u8, line),
            Instruction::Greater =>
                self.write(OpCode::Greater as u8, line),
            Instruction::Less =>
                self.write(OpCode::Less as u8, line),
            Instruction::Add =>
                self.write(OpCode::Add as u8, line),
            Instruction::Subtract =>
                self.write(OpCode::Subtract as u8, line),
            Instruction::Multiply =>
                self.write(OpCode::Multiply as u8, line),
            Instruction::Divide =>
                self.write(OpCode::Divide as u8, line),
            Instruction::Return => 
                self.write(OpCode::Return as u8, line),
            Instruction::Print =>
                self.write(OpCode::Print as u8, line),
            Instruction::Pop =>
                self.write(OpCode::Pop as u8, line),
            Instruction::Jump { jump_distance: offset } => {
                self.write(OpCode::Jump as u8, line);
                self.write_u16(offset, line);
            },
            Instruction::JumpIfFalse { jump_distance: offset } => {
                self.write(OpCode::JumpIfFalse as u8, line);
                self.write_u16(offset, line);
            },
            Instruction::Loop { jump_distance: offset } => {
                self.write(OpCode::Loop as u8, line);
                self.write_u16(offset, line);
            },
            Instruction::Call { num_args } => {
                self.write(OpCode::Call as u8, line);
                self.write(num_args, line);
            }
        }

        let next_offset = self.code.len();

        (start_offset, next_offset)
    }

    pub fn update_jump_offset(&mut self, offset: usize, jump_delta: u16) {
        let bytes = jump_delta.to_be_bytes();
        self.code[offset + 1] = bytes[0];
        self.code[offset + 2] = bytes[1];
    }

    pub fn read(&self, offset: usize) -> Option<&u8> {
        self.code.get(offset)
    }

    pub fn read_n_bytes(&self, offset: usize, n: usize) -> &[u8] {
        &self.code[offset..(offset + n)]
    }

    pub fn read_u16(&self, offset: usize) -> u16 {
        let bytes_slice = self.read_n_bytes(offset, 2);
        let mut bytes: [u8; 2] = [0; 2];
        for (i, byte) in bytes_slice.iter().enumerate() {
            bytes[i] = *byte;
        }
        u16::from_be_bytes(bytes) 
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let bytes_slice = self.read_n_bytes(offset, 4);
        let mut bytes: [u8; 4] = [0; 4];
        for (i, byte) in bytes_slice.iter().enumerate() {
            bytes[i] = *byte;
        }
        u32::from_be_bytes(bytes) 
    }

    pub fn read_instruction(&self, offset: usize) -> Option<(Instruction, usize)> {

        let op_code_opt = self.read(offset);
        let mut next_offset = offset + 1;

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
                let value_idx_opt = self.read(next_offset);
                next_offset += 1;
                match value_idx_opt {
                    Some(value_idx) => 
                        Some((Instruction::Constant { value_idx: value_idx.clone() }, next_offset)),
                    None => None
                }
            },
            OpCode::ConstantLong => {
                let value_idx = self.read_u32(next_offset);
                next_offset += 4;
                Some((Instruction::ConstantLong { value_idx: value_idx.clone() }, 
                    next_offset))
            },
            OpCode::Nil => 
                Some((Instruction::Nil, next_offset)),
            OpCode::True => 
                Some((Instruction::True, next_offset)),
            OpCode::False => 
                Some((Instruction::False, next_offset)),
            OpCode::Negate =>
                Some((Instruction::Negate, next_offset)),
            OpCode::Not =>
                Some((Instruction::Not, next_offset)),
            OpCode::Equal =>
                Some((Instruction::Equal, next_offset)),
            OpCode::Greater =>
                Some((Instruction::Greater, next_offset)),
            OpCode::Less =>
                Some((Instruction::Less, next_offset)),
            OpCode::Add =>
                Some((Instruction::Add, next_offset)),
            OpCode::Subtract =>
                Some((Instruction::Subtract, next_offset)),
            OpCode::Multiply =>
                Some((Instruction::Multiply, next_offset)),
            OpCode::Divide =>
                Some((Instruction::Divide, next_offset)),
            OpCode::Return => 
                Some((Instruction::Return, next_offset)),
            OpCode::Print => 
                Some((Instruction::Print, next_offset)),
            OpCode::Pop =>
                Some((Instruction::Pop, next_offset)),
            OpCode::DefineGlobal => {
                    let global_idx = self.read_u32(next_offset);
                    next_offset += 4;
                    Some((Instruction::DefineGlobal{ global_idx }, next_offset))
                },
            OpCode::GetGlobal => {
                    let global_idx = self.read_u32(next_offset);
                    next_offset += 4;
                    Some((Instruction::GetGlobal{ global_idx }, next_offset))
                },
            OpCode::SetGlobal => {
                    let global_idx = self.read_u32(next_offset);
                    next_offset += 4;
                    Some((Instruction::SetGlobal{ global_idx }, next_offset))
                },
            OpCode::GetLocal => {
                    let local_idx = self.read_u32(next_offset);
                    next_offset += 4;
                    Some((Instruction::GetLocal{ local_idx }, next_offset))
                },
            OpCode::SetLocal => {
                    let local_idx = self.read_u32(next_offset);
                    next_offset += 4;
                    Some((Instruction::SetLocal{ local_idx }, next_offset))
            },
            OpCode::Jump => {
                let offset = self.read_u16(next_offset);
                next_offset += 2;
                Some((Instruction::Jump{jump_distance: offset}, next_offset))
            },
            OpCode::JumpIfFalse => {
                let offset = self.read_u16(next_offset);
                next_offset += 2;
                Some((Instruction::JumpIfFalse{jump_distance: offset}, next_offset))
            },
            OpCode::Loop => {
                let offset = self.read_u16(next_offset);
                next_offset += 2;
                Some((Instruction::Loop{jump_distance: offset}, next_offset))
            },
            OpCode::Call => {
                let num_args_opt = self.read(next_offset);
                next_offset += 1;
                match num_args_opt {
                    Some(num_args) => {
                        let num_args = num_args.clone();
                        Some((Instruction::Call { num_args }, next_offset))
                    },
                    None => None,
                }
            },
        }
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        if let Value::Str(sref) = &value {
            let s = sref.get_string();
            if self.string_idxs.contains_key(&s) {
                self.string_idxs[&s]
            } else {
                self.values.push(value);
                let value_idx = self.values.len() - 1;
                self.string_idxs.insert(s, value_idx);
                value_idx
            }
        } else {
            self.values.push(value);
            self.values.len() - 1    
        }
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

    pub fn instruction_iter(self: &Chunk) -> InstructionIter {
        InstructionIter { 
            chunk: self,
            offset: 0 }
    }

    pub fn size(&self) -> usize {
        self.code.len()
    }

}

pub struct InstructionIter <'a> {
    chunk: &'a Chunk,
    offset: usize
}

impl <'a> Iterator for InstructionIter<'a> {
    
    type Item = (Instruction, usize);

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some((instr, new_offset)) = self.chunk.read_instruction(self.offset) {
            let item = Some((instr, self.offset));
            self.offset = new_offset;
            item
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::instruction::Instruction::*;
    use crate::backend::chunk::Chunk;
    
    #[test]
    fn iterate_chunk() {
     
        let mut chunk = Chunk::new();
        chunk.write_instruction(Constant{ value_idx: 42 }, 1);
        chunk.write_instruction(Constant { value_idx: 23 }, 2);
        chunk.write_instruction(Return, 2);
        chunk.write_instruction(ConstantLong { value_idx: 625 }, 3);    
        
        for (instr, offset) in chunk.instruction_iter() {
            println!("{:04} {}", offset, instr);
        }
    }

}