use super::instruction::Instruction;
use super::chunk::Chunk;


pub fn disassemble(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut line_opt: Option<i32> = None;
    let mut line_info: String;

    for (instr, offset) in chunk.instruction_iter() {
        let curr_line = chunk.get_line(offset).unwrap();
        line_info = if line_opt.is_none() || line_opt.unwrap() != curr_line {
            format!("{:04}", curr_line)
        } else {
            "   |".to_string()
        };

        println!("{:04} {} {}", 
            offset, 
            line_info, 
            disassemble_instruction(chunk, &instr));

        line_opt = Some(curr_line);
    }

}

pub fn disassemble_instruction(chunk: &Chunk, instr: &Instruction) -> String {
    match instr {
        Instruction::Constant { value_idx } => 
            disassemble_constant(chunk, value_idx),
        Instruction::ConstantLong { value_idx } => 
            disassemble_constant_long(chunk, value_idx),
        Instruction::DefineGlobal { global_idx } => 
            disassemble_def_global(chunk, global_idx),
        Instruction::GetGlobal { global_idx } => 
            disassemble_get_global(chunk, global_idx),
        Instruction::SetGlobal { global_idx } => 
            disassemble_set_global(chunk, global_idx),
        Instruction::Nil =>
            "OP_NIL".to_string(),
        Instruction::True =>
            "OP_TRUE".to_string(),
        Instruction::False =>
            "OP_FALSE".to_string(),
        Instruction::Negate =>
            "OP_NEGATE".to_string(),
        Instruction::Not =>
            "OP_NOT".to_string(),
        Instruction::Equal =>
            "OP_EQUAL".to_string(),
        Instruction::Greater =>
            "OP_GREATER".to_string(),
        Instruction::Less =>
            "OP_LESSER".to_string(),
        Instruction::Add =>
            "OP_ADD".to_string(),
        Instruction::Subtract =>
            "OP_SUBTRACT".to_string(),
        Instruction::Multiply =>
            "OP_MULTIPLY".to_string(),
        Instruction::Divide =>
            "OP_DIVIDE".to_string(),
        Instruction::Return => 
            "OP_RETURN".to_string(),
        Instruction::Print =>
            "OP_PRINT".to_string(),
        Instruction::Pop =>
            "OP_POP".to_string(),
    } 
}


fn disassemble_constant(chunk: &Chunk, value_idx: &u8) -> String {
    let value = chunk.read_value(*value_idx as usize).unwrap();
    format!("{:<16} {:04} ({})", "OP_CONSTANT", value_idx, value)
}

fn disassemble_constant_long(chunk: &Chunk, value_idx: &u32) -> String {
    let value = chunk.read_value(*value_idx as usize).unwrap();
    format!("{:<16} {:04} ({})", "OP_CONSTANT_LONG", value_idx, value)
}

fn disassemble_def_global(chunk: &Chunk, global_idx: &u32) -> String {
    let value = chunk.read_value(*global_idx as usize).unwrap();
    format!("{:<16} {:04} ({})", "OP_DEFINE_GLOBAL", global_idx, value)
}

fn disassemble_get_global(chunk: &Chunk, global_idx: &u32) -> String {
    let value = chunk.read_value(*global_idx as usize).unwrap();
    format!("{:<16} {:04} ({})", "OP_GET_GLOBAL", global_idx, value)
}

fn disassemble_set_global(chunk: &Chunk, global_idx: &u32) -> String {
    let value = chunk.read_value(*global_idx as usize).unwrap();
    format!("{:<16} {:04} ({})", "OP_SET_GLOBAL", global_idx, value)
}

#[cfg(test)]
mod tests {

    use crate::backend::instruction::OpCode;
    use crate::backend::value::Value;
    use crate::backend::chunk::Chunk;
    use super::*;
    
    #[test]
    fn disassemble_chunk() {
     
        let mut chunk = Chunk::new();

        let fourty_two = chunk.add_value(Value::Number(42.0));
        let twenty_three = chunk.add_value(Value::Number(23.0));

        for i in 2..1000 {
            chunk.add_value(Value::Number((i as f64).sqrt()));
        }
    
        chunk.write(OpCode::Constant as u8, 1);
        chunk.write(fourty_two as u8, 1);
        chunk.write(OpCode::Constant as u8, 2);
        chunk.write(twenty_three as u8, 2);
        chunk.write(OpCode::Return as u8, 2);
        chunk.write(OpCode::ConstantLong as u8, 3);
        chunk.write_long(625, 3);
        
        disassemble(&chunk, "test chunk");
    }

}
