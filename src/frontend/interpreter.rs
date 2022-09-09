use std::{io::{self, Read}, path::Path, fs::File};
use crate::{backend::{InterpretResult, heap::HeapManager, vm::{VM, CallFrame}}, frontend::compiler::Compiler};

pub fn repl() {

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        line.clear();
        match stdin.read_line(&mut line) {
            Ok(_) => {
                interpret(&line);
                ()
            },
            Err(_) => break,
        }
    }

}

pub fn run_file(file_path: &str) -> Result<(), i32>{

    let mut file = match File::open(Path::new(file_path)) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not open file {}", file_path);
            return Err(74);
        },
    };

    let mut source = String::new();

    match file.read_to_string(&mut source) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Could not open file {}", file_path);
            return Err(74);
        }
    }
    
    match interpret(&source) {
        InterpretResult::Ok => Ok(()),
        InterpretResult::CompileError => Err(65),
        InterpretResult::RuntimeError => Err(70),
    }
    
}

pub fn interpret(source: &str) -> InterpretResult {

    let heap_manager = HeapManager::new_rc_refcell();
    let mut compiler = Compiler::new_with_heap_mgr(source, &heap_manager);

    let ret = if let Some(func_data) = compiler.compile() {
        let frame = CallFrame::new_top_with_func_data(func_data);
        VM::new_with_frame(frame).run()
    } else {
        heap_manager.borrow_mut().free_all();
        InterpretResult::CompileError
    };
    
    heap_manager.borrow_mut().free_all();

    ret
}