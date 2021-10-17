use crate::chunk;

pub fn disassemble_chunk(chunk: &chunk::Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

fn disassemble_instruction(chunk: &chunk::Chunk, offset: usize) -> usize {
    println!("{}", offset);

    let instruction = &chunk.code[offset];

    match instruction {
        chunk::OpCode::OpReturn => simple_instruction("OP_RETURN", offset)
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}