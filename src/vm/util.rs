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
                OpCode::Return => Some((offset + 1, "OP_RETURN".to_string())),
            },
            Err(err) => Some((offset + 1, err))
        }
    } else {
        None
    }
}
