use crate::vm::VM;

#[derive(Debug)]
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

pub fn scan_token(vm: &VM, scanner: &mut Scanner) -> Token {
    skip_whitespace(scanner);

    if is_at_end(&scanner) {
        return make_token(&scanner, TokenType::EOF);
    }

    let c: char = advance(scanner);

    return match c {
        '(' => make_token(&scanner, TokenType::LeftParen),
        ')' => make_token(&scanner, TokenType::RightParen),
        '{' => make_token(&scanner, TokenType::LeftBrace),
        '}' => make_token(&scanner, TokenType::RightBrace),
        ';' => make_token(&scanner, TokenType::Semicolon),
        ',' => make_token(&scanner, TokenType::Comma),
        '.' => make_token(&scanner, TokenType::Dot),
        '-' => make_token(&scanner, TokenType::Minus),
        '+' => make_token(&scanner, TokenType::Plus),
        '/' => make_token(&scanner, TokenType::Slash),
        '*' => make_token(&scanner, TokenType::Star),
        '!' => {
            if match_token(scanner, '=') {
                make_token(&scanner, TokenType::BangEqual)
            } else {
                make_token(&scanner, TokenType::Bang)
            }
        },
        '=' => {
            if match_token(scanner, '=') {
                make_token(&scanner, TokenType::EqualEqual)
            } else {
                make_token(&scanner, TokenType::Equal)
            }
        },
        '<' => {
            if match_token(scanner, '=') {
                make_token(&scanner, TokenType::LessEqual)
            } else {
                make_token(&scanner, TokenType::Less)
            }
        },
        '>' => {
            if match_token(scanner, '=') {
                make_token(&scanner, TokenType::GreaterEqual)
            } else {
                make_token(&scanner, TokenType::Greater)
            }
        },
        _ => error_token(&scanner, "Unexpected Character")
    };
}

fn is_at_end(scanner: &Scanner) -> bool {
    scanner.current >= scanner.source.len() - 2 || scanner.source[scanner.current] == '\0'
}

fn advance(scanner: &mut Scanner) -> char {
    scanner.current += 1;
    return scanner.source[scanner.current - 1];
}

fn match_token(scanner: &mut Scanner, expected: char) -> bool {
    if is_at_end(scanner) {
        return false;
    }

    if scanner.source[scanner.current] != expected {
        return false;
    }

    scanner.current += 1;
    return true;
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

fn skip_whitespace(scanner: &mut Scanner) {
    loop {
        let c = peek(scanner);

        match c {
            ' ' => { advance(scanner); break; },
            '\r' => { advance(scanner); break; },
            '\t' => { advance(scanner); break; },
            '\n' => { scanner.line += 1; advance(scanner); break; },

            '/' => {
                if peek_next(scanner) == '/' {
                    while peek(scanner) != '\n' && !is_at_end(scanner) {
                        advance(scanner);
                    } 
                } else {
                    return;
                }
            },

            _ => { return; }
        };
    }
}

fn peek(scanner: &Scanner) -> char {
    if is_at_end(scanner) {
        return '\0';
    }

    return scanner.source[scanner.current];
}

fn peek_next(scanner: &Scanner) -> char {
    if is_at_end(scanner) {
        return '\0';
    } else {
        return scanner.source[scanner.current + 1];
    }
}