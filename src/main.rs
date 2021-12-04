use std::env;
use std::io::Write;

use chunk::{Chunk, OpCode};
use disassembler::disassemble_chunk;
use vm::VM;
use vm::interpret;
use vm::InterpretResult;

mod chunk;
mod disassembler;
mod value;
mod vm;
mod compiler;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    let vm = VM::new();

    if args.len() == 1 {
        repl(&vm);
    } else if args.len() == 2 {
        let path = &args[1];
        run_file(&vm, path);
    } else {
        println!("Usage: rlox [script]");
    }
}

fn repl(vm: &VM) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        vm::interpret(vm, &input);
    }
}

fn run_file(vm: &VM, path: &str) {
    let mut buf = String::new();
    let mut file = std::fs::read_to_string(path).unwrap();

    let result: InterpretResult = vm::interpret(vm, &file);

    match result {
        InterpretResult::Ok => (),
        InterpretResult::RuntimeError => {
            eprintln!("Runtime Error");
        }
        InterpretResult::CompileError => {
            eprintln!("Compile Error");
        }
    }
}