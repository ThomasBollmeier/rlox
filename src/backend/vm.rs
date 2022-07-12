use super::chunk::Chunk;

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

struct VM {
    chunk: Chunk,
    ip: usize, // instruction pointer
}

impl VM {
    pub fn new() -> VM {
        VM::new_with_chunk(Chunk::new())
    }

    pub fn new_with_chunk(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
        }
    }

    pub fn interpret() -> InterpretResult {
        InterpretResult::Ok
    }
}

#[cfg(test)]
mod tests {

}