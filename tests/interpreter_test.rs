use rlox::{frontend::interpreter, backend::InterpretResult};

#[test]
fn run_file() {
    let result = interpreter::run_file("tests/examples/test.lox");
    assert!(result.is_err());
}

#[test]
fn interpret_string() {
    let source = "\"Hallo Welt!\"";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_string_equal() {
    let source = "\"Hallo Welt!\" == \"Hallo Welt!\"";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_string_concat() {
    let source = "\"Hallo\" + \" Welt!\"";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}