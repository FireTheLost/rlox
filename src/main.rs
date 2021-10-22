use std::env;
use chunk::{Chunk, OpCode};
use disassembler::disassemble_chunk;

mod chunk;
mod disassembler;
mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut chunk: Chunk = Chunk::new();

    let constant: usize = chunk.add_constant(1.2);
    chunk.write_chunk(OpCode::OpConstant(constant), 123);

    chunk.write_chunk(OpCode::OpReturn, 123);

    disassemble_chunk(&chunk, "Test Chunk");
}