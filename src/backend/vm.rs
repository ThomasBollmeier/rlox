use std::{cell::{RefCell, RefMut, Ref}, collections::HashMap};
use super::{chunk::Chunk, instruction::Instruction, value::Value, util::disassemble_instruction, heap::HeapRef, objects::{FunData, NativeFunData}};

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct CallFrame {
    func_data: FunData,
    ip: usize, // <-- instruction pointer
    stack_base: usize, // <-- base offset in stack
    caller_line: i32, 
}

impl CallFrame {

    pub fn new_top() -> CallFrame {
        Self::new_top_with_func_data(FunData::new_top())
    }

    pub fn new_top_with_func_data(func_data: FunData) -> CallFrame {
        CallFrame { 
            func_data, 
            ip: 0, 
            stack_base: 0, 
            caller_line: 0,
        }
    }

    pub fn new(func_data: FunData, ip: usize, stack_base: usize, caller_line: i32) -> CallFrame {
        CallFrame { func_data, ip, stack_base, caller_line }
    }
}

pub struct VM {
    frames: Vec<CallFrame>,
    stack: RefCell<Vec<Value>>,
    globals: RefCell<HashMap<String, Value>>,
}

impl VM {
    pub fn new() -> VM {
        VM::new_with_frame(CallFrame::new_top())
    }

    pub fn new_with_frame(frame: CallFrame) -> VM {
        VM {
            frames: vec![frame],
            stack: RefCell::new(Vec::new()),
            globals: RefCell::new(HashMap::new()),
        }
    }

    pub fn define_native_fun(&mut self, native: &HeapRef<NativeFunData>) {
        let name = native.get_content().name;
        self.globals.borrow_mut().insert(name, Value::NativeFun(native.clone()));
    }

    fn current_chunk(&self) -> Ref<Chunk> {
        let current_frame = self.frames.last().unwrap();
        current_frame.func_data.borrow_chunk()
    } 

    fn current_chunk_mut(&mut self) -> RefMut<Chunk> {
        let current_frame = self.frames.last_mut().unwrap();
        current_frame.func_data.borrow_chunk_mut()
    } 

    fn current_ip(&self) -> usize {
        let current_frame = self.frames.last().unwrap();
        let ret = current_frame.ip;
        ret
    } 

    fn current_base(&self) -> usize {
        let current_frame = self.frames.last().unwrap();
        let ret = current_frame.stack_base;
        ret
    } 

    fn set_current_ip(&mut self, ip: usize) {
        let current_frame = self.frames.last_mut().unwrap();
        current_frame.ip = ip;
    }
    
    pub fn add_instruction(&mut self, instr: Instruction, line: i32) {
        let mut chunk = self.current_chunk_mut();
        chunk.write_instruction(instr, line);
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        let mut chunk = self.current_chunk_mut();
        chunk.add_value(value)
    }

    pub fn run(&mut self) -> InterpretResult {

        loop {

            let instr_offs_opt = {
                let chunk = self.current_chunk();
                let ip = self.current_ip();
                chunk.read_instruction(ip)
            };

            if instr_offs_opt.is_none() {
                break;
            }

            let (instr, next_offset) = instr_offs_opt.unwrap();

            if cfg!(trace_run) {
                let chunk = self.current_chunk();
                self.show_stack();
                println!("{}", disassemble_instruction(&chunk, &instr));
            }

            let offset = self.current_ip();
            self.set_current_ip(next_offset);

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
                Instruction::SetGlobal { global_idx } =>
                    self.interpret_set_global(global_idx as usize, self.get_line(offset)),
                Instruction::GetLocal { local_idx } =>
                    self.interpret_get_local(local_idx as usize),
                Instruction::SetLocal { local_idx } =>
                    self.interpret_set_local(local_idx as usize),
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
                Instruction::Jump { jump_distance } =>
                    self.interpret_jump(offset, jump_distance), 
                Instruction::JumpIfFalse { jump_distance } =>
                    self.interpret_jump_if_false(offset, jump_distance), 
                Instruction::Loop { jump_distance } =>
                    self.interpret_loop(offset, jump_distance),
                Instruction::Call { num_args } =>
                    self.interpret_call(num_args, self.get_line(offset)),
            };

            if let Some(result) = result {
                return result;
            }

        }

        if self.stack.borrow().is_empty() {
            InterpretResult::Ok
        } else {
            InterpretResult::RuntimeError
        }
        
    }

    fn get_line(&self, offset: usize) -> i32 {
        self.current_chunk().get_line(offset).unwrap_or(1)
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
        self.print_callstack(line);
    }

    fn print_callstack(&self, line: i32) {
        let mut call_line = line;
        for frame in self.frames.iter().rev() {
            let mut fun_name = frame.func_data.name.clone();
            if fun_name.is_empty() {
                fun_name = "script".to_string();
            } else {
                fun_name = format!("{fun_name}()");
            };
            eprintln!("[line {}] in {}", call_line, fun_name);
            call_line = frame.caller_line;
        }
    }

    fn interpret_return(&mut self) -> Option<InterpretResult> {
        if self.frames.len() == 1 {
            return if self.stack.borrow().is_empty() {
                Some(InterpretResult::Ok)
            } else {
                Some(InterpretResult::RuntimeError)
            }    
        }

        if self.frames.len() > 1 {
            let frame = self.frames.pop().unwrap();
            let result = self.pop(); 
            let stack_size = self.stack.borrow().len();
            let num_pops = stack_size - frame.stack_base;

            for _ in 0..num_pops {
                self.pop();
            }

            self.push(&result);
        }

        None
    }

    fn interpret_constant(&self, value_idx: usize) -> Option<InterpretResult> {
        let chunk = self.current_chunk();
        let value = chunk
                .read_value(value_idx)
                .unwrap();
        self.push(value);
        None
    }

    fn interpret_def_global(&self, global_idx: usize, line: i32) -> Option<InterpretResult> {
        let chunk = self.current_chunk();
        let value = chunk
                .read_value(global_idx)
                .unwrap();

        match value {
            Value::Str(s) => {
                let varname = s.get_content();
                let value = self.peek(0).unwrap();
                self.globals.borrow_mut().insert(varname, value);
                self.pop();
            },
            _ => {
                self.print_runtime_error(line, "Expected string value.");
                return Some(InterpretResult::RuntimeError);
            }
        }
        
        None
    }

    fn interpret_get_global(&self, global_idx: usize, line: i32) -> Option<InterpretResult> {
        let chunk = self.current_chunk();
        let value = chunk
            .read_value(global_idx)
            .unwrap();

        match value {
            Value::Str(s) => {
                let varname = s.get_content();
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

    fn interpret_set_global(&self, global_idx: usize, line: i32) -> Option<InterpretResult> {
        let chunk = self.current_chunk();
        let value = chunk
            .read_value(global_idx)
            .unwrap();

        match value {
            Value::Str(s) => {
                let varname = s.get_manager().borrow().get_content(s).clone();
                let mut globals = self.globals.borrow_mut();
                if globals.contains_key(&varname) {
                    let new_value = self.peek(0).unwrap();
                    globals.insert(varname, new_value);
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

    fn interpret_get_local(&self, local_idx: usize) -> Option<InterpretResult> {
        let mut stack = self.stack.borrow_mut();
        let absolute_idx = self.current_base() + local_idx;
        let value = stack[absolute_idx].clone();
        stack.push(value);
        None
    }

    fn interpret_set_local(&self, local_idx: usize) -> Option<InterpretResult> {
        let value = self.peek(0).unwrap();
        let absolute_idx = self.current_base() + local_idx;
        self.stack.borrow_mut()[absolute_idx] = value;
        None
    }

    fn interpret_jump(&mut self, offset: usize, jump_distance: u16) -> Option<InterpretResult> {
        self.set_current_ip(offset + jump_distance as usize);
        None
    }

    fn interpret_jump_if_false(&mut self, offset: usize, jump_distance: u16) -> Option<InterpretResult> {
        let condition = self.peek(0).unwrap();
        if Self::is_falsey(&condition) {
            self.set_current_ip(offset + jump_distance as usize);
        }
        None
    }

    fn interpret_loop(&mut self, offset: usize, jump_distance: u16) -> Option<InterpretResult> {
        self.set_current_ip(offset - jump_distance as usize);
        None
    }

    fn interpret_call(&mut self, num_args: u8, line: i32) -> Option<InterpretResult> {
        
        let (value, fun_idx) = {
            let stack = self.stack.borrow();
            let fun_idx = stack.len() - 1 - (num_args as usize);
            (&stack[fun_idx].clone(), fun_idx)
        };

        match value {
            Value::Fun(fun_data) => {
                let fun_data = fun_data.get_content();

                if fun_data.arity != num_args {
                    let message = format!("Expected {} arguments but got {}",
                        fun_data.arity, num_args);
                    self.print_runtime_error(line, &message);
                    return Some(InterpretResult::RuntimeError);    
                }

                let new_frame = CallFrame::new(
                    fun_data, 0, fun_idx, line);
                self.frames.push(new_frame);
            },
            Value::NativeFun(native_fun_data) => {
                let native = native_fun_data.get_content();

                if native.arity != num_args {
                    let message = format!("Expected {} arguments but got {}",
                        native.arity, num_args);
                    self.print_runtime_error(line, &message);
                    return Some(InterpretResult::RuntimeError);    
                }

                let args = self.pop_call_args(num_args);
                self.pop(); // remove native function from stack 
                let result = (native.fun)(args);
                self.push(&result);

            },
            _ => {
                self.print_runtime_error(line, &format!("{} is not a function.", value));
                return Some(InterpretResult::RuntimeError);
            }
        }

        None
    }

    fn pop_call_args(&self, num_args: u8) -> Vec<Value> {
        let mut ret = vec![];
        
        for _ in 0..num_args {
            let arg = self.pop();
            ret.push(arg);
        }
        
        ret.reverse();
        
        ret
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