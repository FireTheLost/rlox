use std::env;

mod chunk;
mod disassembler;
mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut chunk = chunk::Chunk::new();
    chunk.write_chunk(chunk::OpCode::OpReturn);

    disassembler::disassemble_chunk(&chunk, "Test Chunk");
}
