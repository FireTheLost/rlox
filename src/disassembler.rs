use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04}  ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ")
    } else {
        print!("{:04} ", chunk.lines[offset])
    }

    let instruction = &chunk.code[offset];

    match instruction {
        OpCode::OpConstant(constant) => { return constant_instruction("OP_CONSTANT", chunk, offset, constant); }
        OpCode::OpReturn => { return simple_instruction("OP_RETURN", offset); }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize, constant: &usize) -> usize {
    print!("{}     {}  '", name, constant);
    print_value(&chunk.constants.values[offset]);
    println!("'");
    offset + 1
}

fn print_value(value: &f64) {
    print!("{}", value);
}