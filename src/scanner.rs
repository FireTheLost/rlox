use crate::vm::VM;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: i32,
}

pub struct Token {
    pub ttype: TokenType,
    pub start: usize,
    pub length: i32,
    pub line: i32,
    pub lexeme: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    Identifier, TokenString, Number,

    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, EOF
}

pub fn init_scanner(vm: &VM, source: &str) -> Scanner {
    Scanner {
        source: source.chars().collect(),
        start: 0,
        current: 0,
        line: 1,
    }
}

pub fn scan_token(vm: &VM, scanner: &Scanner) -> Token {
    if is_at_end(&scanner) {
        return make_token(&scanner, TokenType::EOF);
    }

    return error_token(&scanner, "Unexpected Character");
}

fn is_at_end(scanner: &Scanner) -> bool {
    return scanner.current >= scanner.source.len();
}

fn make_token(scanner: &Scanner, ttype: TokenType) -> Token {
    Token {
        ttype: ttype,
        start: scanner.start,
        length: (scanner.current - scanner.start) as i32,
        line: scanner.line,
        lexeme: scanner.source[scanner.start..scanner.current].iter().collect(),
    }
}

fn error_token(scanner: &Scanner, message: &str) -> Token {
    Token {
        ttype: TokenType::Error,
        start: scanner.start,
        length: message.len() as i32,
        line: scanner.line,
        lexeme: message.to_string(),
    }
}