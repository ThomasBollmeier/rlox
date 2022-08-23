use std::{cell::RefCell, collections::HashMap};

use super::{chunk::Chunk, instruction::Instruction, value::Value, util::disassemble_instruction, heap::HeapRef};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    stack: RefCell<Vec<Value>>,
    globals: RefCell<HashMap<String, Value>>,
}

impl VM {
    pub fn new() -> VM {
        VM::new_with_chunk(Chunk::new())
    }

    pub fn new_with_chunk(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: RefCell::new(Vec::new()),
            globals: RefCell::new(HashMap::new()),
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
                Instruction::DefineGlobal { global_idx } => 
                    self.interpret_def_global(global_idx as usize, self.get_line(offset)),
                Instruction::GetGlobal { global_idx } =>
                    self.interpret_get_global(global_idx as usize, self.get_line(offset)),
                Instruction::Nil =>
                    self.interpret_nil(),
                Instruction::True =>
                    self.interpret_true(),
                Instruction::False =>
                    self.interpret_false(),
                Instruction::Negate => 
                    self.interpret_negate(self.get_line(offset)),
                Instruction::Not =>
                    self.interpret_not(),
                Instruction::Equal => 
                    self.interpret_equal(),
                Instruction::Add |
                Instruction::Subtract |
                Instruction::Multiply |
                Instruction::Divide |
                Instruction::Greater |
                Instruction::Less => 
                    self.interpret_binary(&instr, self.get_line(offset)),
                Instruction::Print =>
                    self.interpret_print(),
                Instruction::Pop =>
                    self.interpret_pop(),
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
        None
    }

    fn interpret_constant(&self, value_idx: usize) -> Option<InterpretResult> {
        let value = 
            self.chunk
                .read_value(value_idx)
                .unwrap();
        self.push(value);
        None
    }

    fn interpret_def_global(&self, global_idx: usize, line: i32) -> Option<InterpretResult> {
        let value = 
            self.chunk
                .read_value(global_idx)
                .unwrap();

        match value {
            Value::Str(s) => {
                let varname = s.get_manager().borrow().deref(s).clone();
                let value = self.pop();
                self.globals.borrow_mut().insert(varname, value);
            },
            _ => {
                self.print_runtime_error(line, "Expected string value.");
                return Some(InterpretResult::RuntimeError);
            }
        }
        
        None
    }

    fn interpret_get_global(&self, global_idx: usize, line: i32) -> Option<InterpretResult> {
        let value = 
            self.chunk
                .read_value(global_idx)
                .unwrap();

        match value {
            Value::Str(s) => {
                let varname = s.get_manager().borrow().deref(s).clone();
                let globals = self.globals.borrow();
                let varvalue = globals.get(&varname);
                if varvalue.is_some() {
                    self.push(varvalue.unwrap());
                } else {
                    self.print_runtime_error(line, 
                        &format!("Undefined variable '{}'.", varname));
                    return Some(InterpretResult::RuntimeError);
                }
            },
            _ => {
                self.print_runtime_error(line, "Expected string value.");
                return Some(InterpretResult::RuntimeError);
            }
        }
        
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

    fn interpret_not(&self) -> Option<InterpretResult> {
        let value = self.pop();
        self.push(&Value::Bool(Self::is_falsey(&value)));
        None
    }

    fn interpret_equal(&self) -> Option<InterpretResult> {
        let b = self.pop();
        let a = self.pop();
        self.push(&Value::Bool(a == b));
        None
    }

    fn interpret_print(&self) -> Option<InterpretResult> {
        let value = self.pop();
        println!("{}", value);
        None
    }

    fn interpret_pop(&self) -> Option<InterpretResult> {
        self.pop();
        None
    }

    fn is_falsey(value: &Value) -> bool {
        match value {
            Value::Nil => true,
            Value::Bool(false) => true,
            _ => false
        }
    }

    fn interpret_binary(&self, instr: &Instruction, line: i32) -> Option<InterpretResult> {
        
        let val_b = self.peek(0).unwrap();
        let val_a = self.peek(1).unwrap();
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut new_string_opt: Option<HeapRef<String>> = None;

        match (val_a, val_b) {
            (Value::Number(aa), Value::Number(bb)) => {
                (a, b) = (aa, bb);
            },
            (Value::Str(a_ref), Value::Str(b_ref)) => {
                match instr { 
                    Instruction::Add => new_string_opt = Some(a_ref.concat(&b_ref)),
                    _ => {
                        self.print_runtime_error(line, "Operator not supported for strings.");
                        return Some(InterpretResult::RuntimeError);
                    },
                }
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
                if new_string_opt.is_none() {
                    self.push(&Value::Number(a + b));
                } else {
                    self.push(&Value::Str(new_string_opt.unwrap()));
                }
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
            Instruction::Greater => {
                self.push(&Value::Bool(a > b));
                None
            },
            Instruction::Less => {
                self.push(&Value::Bool(a < b));
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