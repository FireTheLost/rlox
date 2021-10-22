use crate::value::ValueArray;

pub enum OpCode {
    OpReturn,
    OpConstant(usize)
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: ValueArray,
    pub lines: Vec<u32>
}

impl Chunk{
    pub fn new() -> Chunk  {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::new(),
            lines: Vec::new()
        }
    }

    pub fn write_chunk(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.write_value_array(value);
        self.constants.values.len() - 1
    }
}