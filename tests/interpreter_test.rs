use rlox::{frontend::interpreter, backend::InterpretResult};

#[test]
fn run_file() {
    let result = interpreter::run_file("tests/examples/test.lox");
    assert!(result.is_err());
}

#[test]
fn interpret_string() {
    let source = "\"Hallo Welt!\";";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_string_equal() {
    let source = "\"Hallo Welt!\" == \"Hallo Welt!\";";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_string_concat() {
    let source = "\"Hallo\" + \" Welt!\";";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_print() {
    let source = "print 41 + 1;";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_def_global() {

    let source = "
        var beverage = \"cafe au lait\";
        print \"beignets with \" + beverage;
    ";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_global_assignment() {

    let source = "
        var breakfast;
        var beverage = \"cafe au lait\";
        breakfast = \"beignets with \"; 
        print breakfast + beverage;
    ";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_def_local() {
    
    let source = "
        {
            var a = 41;
            var b = a + 1;
            print b;
        }
    ";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);

}