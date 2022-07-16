use super::{chunk::Chunk, instruction::Instruction, value::Value, util::disassemble_instruction};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
}

impl VM {
    pub fn new() -> VM {
        VM::new_with_chunk(Chunk::new())
    }

    pub fn new_with_chunk(chunk: Chunk) -> VM {
        VM {
            chunk,
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction, line: i32) {
        self.chunk.write_instruction(instr, line);
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.chunk.add_value(value)
    }

    pub fn run(&mut self) -> InterpretResult {
        for (instr, _) in self.chunk.instruction_iter() {
            
            if cfg!(trace_run) {
                println!("{}", disassemble_instruction(&self.chunk, &instr));
            }

            match instr {
                Instruction::Return => return InterpretResult::Ok,
                Instruction::Constant { value_idx } => 
                    self.interpret_constant(value_idx),
                Instruction::ConstantLong { value_idx } => 
                    self.interpret_constant_long(value_idx),
            }
        }
        InterpretResult::Ok
    }

    fn interpret_constant(&self, value_idx: u8) {
        let value = 
            self.chunk
                .read_value(value_idx as usize)
                .unwrap();
        println!("{}", value);
    }

    fn interpret_constant_long(&self, value_idx: u32) {
        let value = 
            self.chunk
                .read_value(value_idx as usize)
                .unwrap();
        println!("{}", value);
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::backend::{instruction::Instruction::*, value::Value};

    #[test]
    fn run() {

        let mut vm = VM::new();

        let val1 = vm.add_value(Value::Number(42.));
        let val2 = vm.add_value(Value::Number(23.));

        vm.add_instruction(Constant { value_idx: val1 as u8 }, 1);
        vm.add_instruction(ConstantLong { value_idx: val2 as u32 }, 2);
        vm.add_instruction(Return, 3);

        vm.run();

    }

}