use std::{io::{self, Read}, path::Path, fs::File, cell::RefCell, rc::Rc};
use crate::{backend::{InterpretResult, heap::{HeapManager, HeapRef}, vm::{VM, CallFrame}, objects::{NativeFn, NativeFunData}, native}, frontend::compiler::Compiler};

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
        let mut vm = VM::new_with_frame(frame);
        set_native_functions(&mut vm, &heap_manager);    
        vm.run()
    } else {
        heap_manager.borrow_mut().free_all();
        InterpretResult::CompileError
    };
    
    heap_manager.borrow_mut().free_all();

    ret
}

fn set_native_functions(vm: &mut VM, hm: &Rc<RefCell<HeapManager>>) {

    vm.define_native_fun(&new_native(
        hm, 
        "sqrt", 
        1,
        native::sqrt 
    ));

    vm.define_native_fun(&new_native(
        hm, 
        "concat", 
        2,
        native::concat 
    ));

}

fn new_native(
    hm: &Rc<RefCell<HeapManager>>, 
    name: &str, 
    arity: u8, 
    native_fn: NativeFn) -> HeapRef<NativeFunData> {

    let native = NativeFunData::new(name, arity, native_fn);
    HeapManager::malloc(hm, native)
}

