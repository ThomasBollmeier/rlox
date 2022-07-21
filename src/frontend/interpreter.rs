use std::io;

pub fn repl() {

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("> ");
        match stdin.read_line(&mut line) {
            Ok(_) => interpret(&line),
            Err(_) => break,
        }
    }

}

pub fn run_file(path: &str) {
    println!("Running {}...", path);
}

fn interpret(line: &str) {
    println!("{}", line);
}