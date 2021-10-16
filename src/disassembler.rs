use crate::chunk::Chunk;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    } 
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize{
    print!("{} ", offset);

    let instruction = &chunk.code[offset];

    match instruction {
        OpReturn => simple_instruction("OP_RETURN", offset),
        _ => {
            println!("OP_UNKOWN");
            offset + 1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}