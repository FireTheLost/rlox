enum OpCode {
    OP_RETURN
}

struct Chunk {
    code: Vec<u8>
}

impl Chunk -> Chunk {
    fn new() {
        Chunk {
            code: Vec::new()
        }
    }

    fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
}