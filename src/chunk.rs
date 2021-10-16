pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
}