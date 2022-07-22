use std::env;
use rlox::frontend::interpreter::{repl, run_file};

fn main() -> Result<(), i32> {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => match run_file(&args[1]) {
            Ok(_) => (),
            Err(error_code) => return Err(error_code)
        },
        _ => {
            eprintln!("Usage: rlox [path]");
            return Err(64);
        },    
    }

    Ok(())
}
