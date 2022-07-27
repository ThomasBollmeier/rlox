use std::{io::{self, Read}, path::Path, fs::File};
use crate::{backend::InterpretResult, frontend::compiler::{self, Compiler}};

pub fn repl() {

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
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

fn interpret(source: &str) -> InterpretResult {

    let mut compiler = Compiler::new(source);

    if !compiler.compile() {
        return InterpretResult::CompileError;
    }  

    //Run VM...

    InterpretResult::Ok
}