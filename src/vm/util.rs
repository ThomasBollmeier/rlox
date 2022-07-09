use super::core::{Chunk, OpCode};

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;

    while let Some((new_offset, instr)) = 
        disassemble_instruction(chunk, offset) {
        
        println!("{:04} {}", offset, instr);

        offset = new_offset;
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> Option<(usize, String)> {
    if let Some(op_code_val) = chunk.read(offset) {
        let result: Result<OpCode, String> = op_code_val.try_into();
        match result {
            Ok(op_code) => match op_code {
                OpCode::Constant => disassemble_constant(chunk, offset),
                OpCode::Return => Some((offset + 1, "OP_RETURN".to_string())),
            },
            Err(err) => Some((offset + 1, err))
        }
    } else {
        None
    }
}

fn disassemble_constant(chunk: &Chunk, offset: usize) -> Option<(usize, String)> {
    let val_idx = chunk.read(offset + 1).unwrap() as usize;
    let value = chunk.read_value(val_idx).unwrap();
    let code = format!("{:<16} {:04} ({})", "OP_CONSTANT", val_idx, value);
    Some((offset + 2, code))
}

#[cfg(test)]
mod tests {
    use crate::vm::core::{Chunk, OpCode};
    use crate::vm::value::Value;
    use super::*;
    
    #[test]
    fn disassemble_chunk() {
     
        let mut chunk = Chunk::new();

        let fourty_two = chunk.add_value(Value::Number(42.0));
        let twenty_three = chunk.add_value(Value::Number(23.0));
    
        chunk.write(OpCode::Return as u8);
        chunk.write(OpCode::Constant as u8);
        chunk.write(fourty_two as u8);
        chunk.write(OpCode::Constant as u8);
        chunk.write(twenty_three as u8);
    
        disassemble(&chunk, "test chunk");
    }

}
