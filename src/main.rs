use rlox::vm::{core::{OpCode, Chunk}, util::disassemble, value::Value};
fn main() {

    let mut chunk = Chunk::new();

    let const_idx = chunk.add_value(Value::Number(42.0));

    chunk.write(OpCode::Return as u8);
    chunk.write(OpCode::Constant as u8);
    chunk.write(const_idx as u8);

    disassemble(&chunk, "test chunk");
}
