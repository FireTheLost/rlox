use std::env;
use chunk::OpCode;

mod chunk;
mod disassembler;
mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut chunk = chunk::Chunk::new();

    let constant: usize = chunk.add_constant(1.2);
    let constant: value::Value = value::Value::Number(constant as f64);
    chunk.write_chunk(OpCode::OpConstant);
    chunk.write_chunk(OpCode::Constant(constant));

    chunk.write_chunk(OpCode::OpReturn);

    disassembler::disassemble_chunk(&chunk, "Test Chunk");
}
