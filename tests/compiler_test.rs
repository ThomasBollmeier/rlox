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