use super::core::{Chunk, OpCode};

pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    let mut line_opt: Option<i32> = None;
    let mut line_info: String;

    while let Some((new_offset, instr, new_line)) = 
        disassemble_instruction(chunk, offset) {
        
        line_info = if line_opt.is_none() || line_opt.unwrap() != new_line {
            format!("{:04}", new_line)
        } else {
            "   |".to_string()
        };
        
        println!("{:04} {} {}", offset, line_info, instr);

        offset = new_offset;
        line_opt = Some(new_line);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> 
    Option<(usize, String, i32)> {
    if let Some(op_code_val) = chunk.read(offset) {
        let line = chunk.read_line(offset);
        let result: Result<OpCode, String> = op_code_val.try_into();
        match result {
            Ok(op_code) => {
                let instr = match op_code {
                    OpCode::Constant => disassemble_constant(chunk, offset),
                    OpCode::Return => disassemble_return(offset),
                };
                match instr {
                    Some((offs, text)) => Some((offs, text, line)),
                    None => None
                }
            },
            Err(err) => Some((offset + 1, err, line))
        }
    } else {
        None
    }
}

fn disassemble_return(offset: usize) -> Option<(usize, String)> {
    Some((offset + 1, "OP_RETURN".to_string()))
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
    
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(fourty_two as u8, 1);
        chunk.write(OpCode::Constant as u8, 2);
        chunk.write(twenty_three as u8, 2);
        chunk.write(OpCode::Return as u8, 2);
        
        disassemble(&chunk, "test chunk");
    }

}
