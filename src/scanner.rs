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
    
    Identifier, String, Number,

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

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn scan_token(vm: &VM, scanner: &mut Scanner) -> Token {
    skip_whitespace(scanner);
    scanner.start = scanner.current;

    if is_at_end(&scanner) {
        return make_token(&scanner, TokenType::EOF);
    }

    let c: char = advance(scanner);

    if is_alpha(c) {
        return identifier(scanner);
    }
    if is_digit(c) {
        return number(scanner);
    }

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

        '"' => string(scanner),

        _ => error_token(&scanner, "Unexpected Character")
    };
}

fn is_at_end(scanner: &Scanner) -> bool {
    scanner.current >= scanner.source.len() || scanner.source[scanner.current] == '\0'
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
            ' ' => { advance(scanner); },
            '\r' => { advance(scanner); },
            '\t' => { advance(scanner); },
            '\n' => { scanner.line += 1; advance(scanner); },

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

fn identifier(scanner: &mut Scanner) -> Token {
    while is_alpha(peek(scanner)) || is_digit(peek(scanner)) {
        advance(scanner);
    }

    return make_token(scanner, identifier_type(scanner));
}

fn identifier_type(scanner: &Scanner) -> TokenType {
    match scanner.source[scanner.start] {
        'a' => return check_keyword(scanner, 1, 2, "nd", TokenType::And),
        'c' => return check_keyword(scanner, 1, 4, "lass", TokenType::Class),
        'e' => return check_keyword(scanner, 1, 3, "lse", TokenType::Else),
        'f' => if scanner.current - scanner.start > 1 {
            match scanner.source[scanner.start + 1] {
                'a' => return check_keyword(scanner, 2, 3, "lse", TokenType::False),
                'o' => return check_keyword(scanner, 2, 1, "r", TokenType::For),
                'u' => return check_keyword(scanner, 2, 1, "n", TokenType::Fun),
                _ => return TokenType::Identifier,
            }
        }
        'i' => return check_keyword(scanner, 1, 1, "f", TokenType::If),
        'n' => return check_keyword(scanner, 1, 2, "il", TokenType::Nil),
        'o' => return check_keyword(scanner, 1, 1, "r", TokenType::Or),
        'p' => return check_keyword(scanner, 1, 4, "rint", TokenType::Print),
        'r' => return check_keyword(scanner, 1, 5, "eturn", TokenType::Return),
        's' => return check_keyword(scanner, 1, 4, "uper", TokenType::Super),
        't' => if scanner.current - scanner.start > 1 {
            match scanner.source[scanner.start + 1] {
                'h' => return check_keyword(scanner, 2, 2, "is", TokenType::This),
                'r' => return check_keyword(scanner, 2, 2, "ue", TokenType::True),
                _ => return TokenType::Identifier,
            }
        }
        'v' => return check_keyword(scanner, 1, 2, "ar", TokenType::Var),
        'w' => return check_keyword(scanner, 1, 4, "hile", TokenType::While),

        _ => ()
    }

    return TokenType::Identifier;
}

fn check_keyword(scanner: &Scanner, start: usize, length: usize, rest: &str, ttype: TokenType) -> TokenType {
    let start_idx = scanner.start + start;
    let end_idx = start_idx + length;

    let substr : String = scanner.source[start_idx..end_idx].into_iter().collect();

    if (scanner.current - scanner.start == start + length) && substr == rest {
        ttype    
    } else {
        TokenType::Identifier
    }
}

fn number(scanner: &mut Scanner) -> Token {
    while is_digit(peek(scanner)) {
        advance(scanner);
    }

    if peek(scanner) == '.' && is_digit(peek_next(scanner)) {
        advance(scanner);

        while is_digit(peek(scanner)) {
            advance(scanner);
        }
    }

    return make_token(scanner, TokenType::Number);
}

fn string(scanner: &mut Scanner) -> Token {
    while peek(scanner) != '"' && !is_at_end(scanner) {
        if peek(scanner) == '\n' {
            scanner.line += 1;
        }

        advance(scanner);
    }

    if is_at_end(scanner) {
        return error_token(scanner, "Unterminated String");
    }

    advance(scanner);

    return make_token(scanner, TokenType::String);
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