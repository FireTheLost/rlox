use crate::chunk;
use crate::chunk::OpCode;
use crate::disassembler::{print_value, disassemble_instruction};

pub struct VM {
    chunk: chunk::Chunk,
    ip: usize,
    stack: Vec<f64>,
    debug: bool
}

pub enum InterpretResult {
    Ok,
    RuntimeError,
    CompileError
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: chunk::Chunk::new(),
            ip: 0,
            stack: Vec::new(),
            debug: true
        }
    }

    pub fn interpret(mut self, chunk: chunk::Chunk) -> InterpretResult{
        self.chunk = chunk;
        return run(self);
    }

    pub fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }
}

pub fn run(mut vm: VM) -> InterpretResult {
    loop {
        if vm.debug {
            println!("          ");
            for slot in vm.stack.iter() {
                print!("[ ");
                print_value(*slot);
                println!(" ]");
            }

            disassemble_instruction(&vm.chunk, vm.ip);
        }

        let instruction = &vm.chunk.code[vm.ip];
        vm.ip += 1;

        match instruction {
            OpCode::OpConstant(pos) => {
                let constant = vm.chunk.constants.values[*pos];
                vm.push(constant);
            },

            OpCode::OpReturn => {
                print_value(vm.pop());
                println!();
                return InterpretResult::Ok;
            }
        }
    }
}