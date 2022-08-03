use std::cell::RefCell;

use super::{chunk::Chunk, instruction::Instruction, value::Value, util::disassemble_instruction};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    stack: RefCell<Vec<Value>>,
}

impl VM {
    pub fn new() -> VM {
        VM::new_with_chunk(Chunk::new())
    }

    pub fn new_with_chunk(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn add_instruction(&mut self, instr: Instruction, line: i32) {
        self.chunk.write_instruction(instr, line);
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.chunk.add_value(value)
    }

    pub fn run(&self) -> InterpretResult {

        for (instr, offset) in self.chunk.instruction_iter() {
            
            if cfg!(trace_run) {
                self.show_stack();
                println!("{}", disassemble_instruction(&self.chunk, &instr));
            }

            let result = match  instr {
                Instruction::Return => 
                    self.interpret_return(),
                Instruction::Constant { value_idx } => 
                    self.interpret_constant(value_idx as usize),
                Instruction::ConstantLong { value_idx } => 
                    self.interpret_constant(value_idx as usize),
                Instruction::Nil =>
                    self.interpret_nil(),
                Instruction::True =>
                    self.interpret_true(),
                Instruction::False =>
                    self.interpret_false(),
                Instruction::Negate => 
                    self.interpret_negate(self.get_line(offset)),
                Instruction::Add |
                Instruction::Subtract |
                Instruction::Multiply |
                Instruction::Divide => 
                    self.interpret_binary(&instr, self.get_line(offset)),
            };

            if let Some(result) = result {
                return result;
            }

        }
        InterpretResult::Ok
    }

    fn get_line(&self, offset: usize) -> i32 {
        self.chunk.get_line(offset).unwrap_or(1)
    }

    fn show_stack(&self) {
        println!();
        println!("=== STACK TOP ===");
        for value in self.stack.borrow().iter().rev() {
            println!("{}", value);
        }
        println!("=== STACK BOTTOM ===");
    }

    fn push(&self, value: &Value) {
        self.stack.borrow_mut().push(value.clone());
    }

    fn pop(&self) -> Value {
        self.stack.borrow_mut().pop().unwrap()
    }

    fn peek(&self, distance: usize) -> Option<Value> {
        let stack = self.stack.borrow();
        let index = stack.len() - distance - 1;
        match stack.get(index) {
            Some(value) => Some(value.clone()),
            None => None,
        } 
    }

    fn print_runtime_error(&self, line: i32, message: &str) {
        eprintln!("{}", message);
        eprintln!("[line {}] in script", line);
    }

    fn interpret_return(&self) -> Option<InterpretResult> {

        let value = self.pop();
        println!("{}", value);

        Some(InterpretResult::Ok)
    }

    fn interpret_constant(&self, value_idx: usize) -> Option<InterpretResult> {
        let value = 
            self.chunk
                .read_value(value_idx)
                .unwrap();
        self.push(value);
        None
    }

    fn interpret_nil(&self) -> Option<InterpretResult> {
        self.push(&Value::Nil);
        None
    }

    fn interpret_true(&self) -> Option<InterpretResult> {
        self.push(&Value::Bool(true));
        None
    }

    fn interpret_false(&self) -> Option<InterpretResult> {
        self.push(&Value::Bool(false));
        None
    }
    fn interpret_negate(&self, line: i32) -> Option<InterpretResult> {
        if let Some(value) = self.peek(0) {
            if let Value::Number(x) = value {
                self.pop();
                self.push(&Value::Number(-x));
                None
            } else {
                self.print_runtime_error(line, "Operand must be a number.");
                Some(InterpretResult::RuntimeError)    
            }
        } else {
            Some(InterpretResult::RuntimeError)
        }

    }

    fn interpret_binary(&self, instr: &Instruction, line: i32) -> Option<InterpretResult> {
        
        let val_b = self.peek(0).unwrap();
        let val_a = self.peek(1).unwrap();
        let a: f64;
        let b: f64;

        match (val_a, val_b) {
            (Value::Number(aa), Value::Number(bb)) => {
                (a, b) = (aa, bb);
            },
            _ => {
                self.print_runtime_error(line, "Operands must be numbers.");
                return Some(InterpretResult::RuntimeError);
            },
        }

        self.pop();
        self.pop();

        match instr {
            Instruction::Add => {
                self.push(&Value::Number(a + b));
                None
            },
            Instruction::Subtract => {
                self.push(&Value::Number(a - b));
                None
            },
            Instruction::Multiply => {
                self.push(&Value::Number(a * b));
                None
            },
            Instruction::Divide => {
                self.push(&Value::Number(a / b));
                None
            },
            _ => Some(InterpretResult::RuntimeError),
        }
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

    #[test]
    fn run_negation() {

        let mut vm = VM::new();
        
        let val = vm.add_value(Value::Number(42.)) as u8;
        
        vm.add_instruction(Constant {value_idx: val}, 123);
        vm.add_instruction(Negate, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

    #[test]
    fn run_addition() {

        let mut vm = VM::new();
        
        let val1 = vm.add_value(Value::Number(1.)) as u8;
        let val2 = vm.add_value(Value::Number(2.)) as u8;
   
        vm.add_instruction(Constant {value_idx: val1}, 123);
        vm.add_instruction(Constant {value_idx: val2}, 123);
        vm.add_instruction(Add, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

    #[test]
    fn run_addition_err() {

        let mut vm = VM::new();
        
        let val1 = vm.add_value(Value::Number(1.)) as u8;
        let val2 = vm.add_value(Value::Bool(true)) as u8;
   
        vm.add_instruction(Constant {value_idx: val1}, 123);
        vm.add_instruction(Constant {value_idx: val2}, 123);
        vm.add_instruction(Add, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

    #[test]
    fn run_subtraction() {

        let mut vm = VM::new();
        
        let val1 = vm.add_value(Value::Number(1.)) as u8;
        let val2 = vm.add_value(Value::Number(2.)) as u8;
   
        vm.add_instruction(Constant {value_idx: val1}, 123);
        vm.add_instruction(Constant {value_idx: val2}, 123);
        vm.add_instruction(Subtract, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

    #[test]
    fn run_multiplication() {

        let mut vm = VM::new();
        
        let val1 = vm.add_value(Value::Number(2.)) as u8;
        let val2 = vm.add_value(Value::Number(3.)) as u8;
   
        vm.add_instruction(Constant {value_idx: val1}, 123);
        vm.add_instruction(Constant {value_idx: val2}, 123);
        vm.add_instruction(Multiply, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

    #[test]
    fn run_division() {

        let mut vm = VM::new();
        
        let val1 = vm.add_value(Value::Number(2.)) as u8;
        let val2 = vm.add_value(Value::Number(3.)) as u8;
   
        vm.add_instruction(Constant {value_idx: val1}, 123);
        vm.add_instruction(Constant {value_idx: val2}, 123);
        vm.add_instruction(Divide, 123);
        vm.add_instruction(Return, 123);

        vm.run();
        
    }

}