use super::value::Value;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    lines: Vec<(i32, usize)>, // source code line mapping
}

impl Chunk {

    pub fn new() -> Chunk {
        Chunk::new_with_capacity(0)
    }
    
    pub fn new_with_capacity(capacity: usize) -> Chunk {
        Chunk {
            code: Vec::with_capacity(capacity),
            values: Vec::new(),
            lines: Vec::new(),
        }
    }
    
    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);

        if !self.lines.is_empty() {
            let previous = self.lines.last_mut().unwrap();
            if previous.0 == line {
                previous.1 += 1;
            } else {
                self.lines.push((line, 1));
            }
        } else {
            self.lines.push((line, 1));
        }
    }    

    pub fn read(&self, offset: usize) -> Option<&u8> {
        self.code.get(offset)
    }

    pub fn add_value(&mut self, value: Value) -> usize {
        self.values.push(value);
        self.values.len() - 1    
    }

    pub fn read_value(&self, offset: usize) -> Option<&Value> {
        self.values.get(offset)
    }

    pub fn get_line(&self, offset: usize) -> Option<i32> {
        let mut total_offset = 0;

        for (line, cnt) in self.lines.iter() {
            total_offset += cnt;
            if offset < total_offset {
                return Some(line.clone());
            }
        }

        None
    }

}