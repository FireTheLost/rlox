use crate::value;

pub enum OpCode {
    OpConstant,
    Constant(value::Value),
    OpReturn
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    constants: value::ValueArray
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: value::ValueArray::new()
        }
    }

    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        self.constants.write_value_array(value);
        self.constants.values.len() - 1
    }
}