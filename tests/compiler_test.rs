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


fn compile_expression(source: &str) {
    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(ok); 

    disassemble(&chunk, "expression");
}