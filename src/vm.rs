use crate::chunk;
use crate::chunk::OpCode;
use crate::disassembler::{print_value, disassemble_instruction};
use crate::compiler::compile;

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

impl PartialEq for InterpretResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InterpretResult::Ok, InterpretResult::Ok) => true,
            (InterpretResult::RuntimeError, InterpretResult::RuntimeError) => true,
            (InterpretResult::CompileError, InterpretResult::CompileError) => true,
            (_, _) => false
        }
    }
}


pub fn interpret(vm: &VM, source: &String) -> InterpretResult{
    let chunk = chunk::Chunk::new();

    if !compile(vm, source, &chunk) {
        return InterpretResult::CompileError;
    }

    vm.chunk = chunk;
    vm.ip = 0;

    let mut result = run(*vm);

    result
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

    pub fn push(&mut self, value: f64) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> f64 {
        self.stack.pop().unwrap()
    }

    fn binary_operator(&mut self, op: OpCode) {
        let b = self.pop();
        let a = self.pop();

        let result = match op {
            OpCode::OpAdd => a + b,
            OpCode::OpSubtract => a - b,
            OpCode::OpMultiply => a * b,
            OpCode::OpDivide => a / b,
            _ => panic!("Invalid Opcode")
        };

        self.push(result);
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

            OpCode::OpAdd => vm.binary_operator(OpCode::OpAdd),

            OpCode::OpSubtract => vm.binary_operator(OpCode::OpSubtract),

            OpCode::OpMultiply => vm.binary_operator(OpCode::OpMultiply),

            OpCode::OpDivide => vm.binary_operator(OpCode::OpDivide),

            OpCode::OpNegate => {
                let value = vm.pop();
                vm.push(-value);
            },

            OpCode::OpReturn => {
                println!();
                print_value(vm.pop());
                println!();
                return InterpretResult::Ok;
            }
        }
    }
}