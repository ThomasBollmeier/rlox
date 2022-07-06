use rlox::vm::core::{OpCode, Chunk};
fn main() {

    let op_code = OpCode::Return; 
    let mut chunk = Chunk::new(0);

    chunk.write(op_code as u8);

    println!("{:?}", chunk);
}
