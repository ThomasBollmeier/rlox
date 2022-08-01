use rlox::{frontend::compiler::Compiler, backend::{chunk::Chunk, util::disassemble}};

#[test]
fn compile_expression() {

    let source = "(1 + 2) * 3 - 4";
    let mut compiler = Compiler::new(source);
    let mut chunk = Chunk::new();

    let ok = compiler.compile(&mut chunk);
    
    assert!(ok); 

    disassemble(&chunk, "expression");

}