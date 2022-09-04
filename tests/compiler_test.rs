use rlox::{frontend::compiler::Compiler, backend::{chunk::Chunk, util::disassemble}};

#[test]
fn compile_arithmetic_expr() {
    compile_expression("(1 + 2) * 3 - 4");
}

#[test]
fn compile_nil() {
    compile_expression("nil");
}

#[test]
fn compile_true() {
    compile_expression("true");
}

#[test]
fn compile_false() {
    compile_expression("false");
}

#[test]
fn compile_comparisons() {
    compile_expression("!(5 - 4 > 3 * 2 == !nil)");
}

#[test]
fn compile_string() {
    compile_expression("\"Hallo Welt!\"");
}

#[test]
fn compile_def_global() {

    let source = "
        var answer = 42;
        print answer;
    ";

    compile_code(source, "global var definition");
}

#[test]
fn compile_assignment() {

    let source = "
        var answer;
        answer = 42;
        print answer;
    ";

    compile_code(source, "assignment");
}

#[test]
fn compile_invalid_assignment() {

    let source = "
        var question = 0;
        var answer = 0;
        question + answer = 42;
        print answer;
    ";

    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(!ok); 
}

#[test]
fn compile_local_def() {

    let source = "        
        {
            var a = 1 + 2;
            var b = a;
            print b;
        }
    ";

    compile_code(source, "local_def");
}

#[test]
fn compile_invalid_local_def() {

    let source = "
        {
            var a = 1;
            var a = 2;
        }
    ";

    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(!ok); 
}

#[test]
fn compile_if() {

    let source = "
        if (1 < 2) {
            print \"Eins ist kleiner!\";
        } else {
            print \"Kann nicht sein.\";
        }
    ";

    compile_code(source, "if");
}

#[test]
fn compile_and() {

    let source = "
        var answer = true and 42;
        print answer;

        answer = false and 23;
        print answer;
    ";

    compile_code(source, "and");
}

#[test]
fn compile_or() {

    let source = "
        var answer = false or 42;
        print answer;

        answer = true or 23;
        print answer;
    ";

    compile_code(source, "or");
}

#[test]
fn compile_while() {

    let source = "
        var counter = 10;
        while (counter >= 0) {
            print counter;
            counter = counter - 1;
        }
    ";

    compile_code(source, "while");
}

#[test]
fn compile_switch() {

    let source = "
        var number = 10;
        switch (number) {
            case 42:
                print \"the answer: \";
                print number;
            default:
                print \"some number\";
        }
    ";

    compile_code(source, "switch");
}

#[test]
fn continue_inside_loop() {

    let source = "
        for (var i = 0; i < 5; i = i + 1)
        {
            var message = \"Hallo!\";
            if (i == 2) continue;
            print i;
        }
    ";

    compile_code(source, "continue_for");
}

#[test]
fn continue_not_allowed_outside_loop() {

    let source = "
        {
            var answer = 42;
            continue;
        }
    ";

    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(!ok); 
}

fn compile_expression(source: &str) {
    // Add semicolon to compile as expression statement
    let expr_statement = source.to_string() + ";";
    compile_code(&expr_statement, "expression");
}

fn compile_code(source: &str, name: &str) {
    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(ok); 

    disassemble(&chunk,name);
}