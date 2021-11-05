use crate::chunk;
use crate::chunk::OpCode;
use crate::disassembler::print_value;

pub struct VM {
    chunk: chunk::Chunk,
    ip: usize
}

pub enum InterpretResult {
    Ok,
    RuntimeError,
    CompileError,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: chunk::Chunk::new(),
            ip: 0
        }
    }

    pub fn interpret(mut self, chunk: chunk::Chunk) -> InterpretResult{
        self.chunk = chunk;
        return run(self);
    }
}

pub fn run(mut vm: VM) -> InterpretResult {
    loop {
        let instruction = &vm.chunk.code[vm.ip];
        vm.ip += 1;

        match instruction {
            OpCode::OpReturn => {
                return InterpretResult::Ok;
            },

            OpCode::OpConstant(pos) => {
                let constant = vm.chunk.constants.values[*pos];
                print_value(constant);
                print!("\n");
            }

            _ => {
                println!("Unimplemented Opcode");
            }
        }
    }
}