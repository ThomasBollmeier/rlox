use rlox::vm::{core::{OpCode, Chunk}, util::dissasemble};
fn main() {

    let op_code = OpCode::Return; 
    let mut chunk = Chunk::new();

    chunk.write(op_code as u8);
    chunk.write(42);

    dissasemble(&chunk, "test chunk");
}
