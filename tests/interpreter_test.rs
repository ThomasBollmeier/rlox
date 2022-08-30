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

#[test]
fn interpret_nested_scopes() {
    
    let source = "
        {
            var a = \"outer\";

            {
                var a = a;
                print a;
                a = \"inner\";
                print a;
            }

            print a;
        }
    ";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_if() {
    
    let source = "
        if (1 < 2) {
            print \"Hallo Welt!\";
        } else {
            print \"Ich werde nicht gedruckt.\";
        }

        if (1 == 2) {
            print \"Keine Ausgabe :-(\";
        }
    ";
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_and() {
    
    let source = "
        var answer = true and 42;
        print answer;

        answer = false and 23;
        print answer;
    ";

    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}

#[test]
fn interpret_or() {
    
    let source = "
        var answer = false or 42;
        print answer;

        answer = true or 23;
        print answer;
    ";
    
    let result = interpreter::interpret(source);
    assert_eq!(result, InterpretResult::Ok);
}