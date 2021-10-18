use crate::chunk::{self, OpCode};
use crate::value;

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
        OpCode::OpReturn => { return simple_instruction("OP_RETURN", offset); }
        OpCode::OpConstant => { return constant_instruction("OP_CONSTANT", chunk, offset); }
        OpCode::Constant(num) => { return constant_instruction("OP_CONSTANT", chunk, offset); }

    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &chunk::Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let OpCode::Constant(constant) = constant;
    print!("{} {} '", name, constant);
    value::print_value(chunk.constants.values[constant]);
    println!("'");
    offset + 2
}