use rlox::frontend::interpreter;

#[test]
fn run_file() {
    let result = interpreter::run_file("tests/examples/test.lox");
    assert!(result.is_err());
}
