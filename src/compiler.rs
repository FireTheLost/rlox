use crate::vm::VM;
use crate::scanner::{init_scanner, scan_token};
use crate::scanner::{Scanner, Token, TokenType};
use crate::chunk::{Chunk, OpCode};

struct Parser {
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

struct Compiler {
    vm: VM,
    scanner: Scanner,
    parser: Parser,
    compiling_chunk: Chunk,
}

fn error(compiler: Compiler, message: &str) {
    error_at(compiler, compiler.parser.previous, message);
}

fn error_at_current(compiler: Compiler, message: &str) {
    error_at(compiler, compiler.parser.current, message);
}

fn error_at(compiler: Compiler, token: Token, message: &str) {
    if compiler.parser.panic_mode {
        return;
    }
    compiler.parser.panic_mode = true;
    eprint!("[line {}] Error", token.line);

    if token.ttype == TokenType::EOF {
        eprint!(" at end");
    } else if token.ttype == TokenType::Error {
    } else {
        eprint!(" at {}", token.start);
    }

    eprintln!(": {}", message);
    compiler.parser.had_error = true;
}

pub fn compile(vm: &VM, source: &str, chunk: &Chunk) -> bool {
    let compiling_chunk = chunk.clone();

    let mut scanner = init_scanner(&vm, source);
    let parser = Parser {
        current: Token {
            ttype: TokenType::Start,
            start: 0,
            length: 0,
            line: 0,
            lexeme: String::new(),
        },
        previous: Token {
            ttype: TokenType::Start,
            start: 0,
            length: 0,
            line: 0,
            lexeme: String::new(),
        },
        had_error: false,
        panic_mode: false,
    };

    let compiler = Compiler {
        vm: *vm.clone(),            
        scanner: scanner,
        parser: parser,
        compiling_chunk: *compiling_chunk,
    };

    advance(&mut compiler);
    expression(&mut compiler, chunk);
    consume(compiler, TokenType::EOF, "Expect End Of Expression");

    end_compiler(&mut compiler);
    return !parser.had_error;
}

fn advance(compiler: &mut Compiler) {
    compiler.parser.previous = compiler.parser.current;

    loop {
        compiler.parser.current = scan_token(&compiler.vm, &mut compiler.scanner);
        if compiler.parser.current.ttype != TokenType::Error {
            break;
        }

        let lexeme = compiler.scanner.source[compiler.parser.current.start..compiler.parser.current.start + compiler.parser.current.length as usize].iter().collect::<String>().as_str();
        error_at_current(*compiler, lexeme);
    }
}

fn consume(compiler: Compiler, ttype: TokenType, message: &str) {
    if compiler.parser.current.ttype == ttype {
        advance(&mut compiler);
        return;
    } else {
        error_at_current(compiler, message);
    }
}

fn emit_byte(compiler: Compiler, byte: OpCode) {
    current_chunk(compiler).write_chunk(byte, compiler.parser.previous.line as u32);
}

fn emit_bytes(compiler: Compiler, byte1: OpCode, byte2: OpCode) {
    emit_byte(compiler, byte1);
    emit_byte(compiler, byte2)
}

fn end_compiler(compiler: &mut Compiler) {
    emit_return(compiler);
}

fn emit_return(compiler: &mut Compiler) {
    emit_byte(*compiler, OpCode::OpReturn);
}

fn current_chunk(compiler: Compiler) -> Chunk {
    return compiler.compiling_chunk;
}