use std::env;
use rlox::frontend::interpreter::{repl, run_file};

fn main() -> Result<(), i32> {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            return Err(64);
        },    
    }

    Ok(())
}
