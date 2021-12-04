use crate::vm::VM;
use crate::scanner::{init_scanner, scan_token};
use crate::scanner::TokenType;

pub fn compile(vm: &VM, source: &str) {
    let mut scanner = init_scanner(&vm, source);

    // println!("{:?}", scanner);

    let mut line = -1;

    loop {
        let token = scan_token(vm, &mut scanner);
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ")
        }

        println!("{:?} {:?}", token.ttype,  token.start);

        if token.ttype == TokenType::EOF {
            break;
        } else if token.ttype == TokenType::Error {
            eprintln!("Error: {}", token.lexeme);
        }
    }
}