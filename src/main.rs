use rlox::vm::{core::{OpCode, Chunk}, util::disassemble};
fn main() {

    let op_code = OpCode::Return; 
    let mut chunk = Chunk::new();

    chunk.write(op_code as u8);
    chunk.write(42);

    disassemble(&chunk, "test chunk");
}
