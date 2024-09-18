use core::fmt;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            if !file_contents.is_empty() {
                let tokens = tokenize(&file_contents);
                match tokens {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{token}");
                        }
                        println!("EOF  null");
                    }
                    Err(tokens) => {
                        for token in tokens {
                            println!("{token}");
                        }

                        println!("EOF  null");
                        std::process::exit(65);
                    }
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "parse" => {
            let tokens = tokenize(&file_contents).unwrap();
            for token in tokens {
                match token {
                    Token::Nil => println!("nil"),
                    Token::True => println!("true"),
                    Token::False => println!("false"),
                    _ => panic!("unhandled token"),
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(file_content: &str) -> Result<Vec<Token>, Vec<Token>> {
    let mut lexical_error = false;
    let mut line_count = 1;
    let mut tokens = Vec::new();

    let mut chars = file_content.chars().peekable();
    while let Some(c) = chars.next() {
        let token = match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '*' => Token::Star,
            '=' => {
                if chars.next_if_eq(&'=').is_some() {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if chars.next_if_eq(&'=').is_some() {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if chars.next_if_eq(&'=').is_some() {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if chars.next_if_eq(&'=').is_some() {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '/' => {
                // We ignore the rest of the line
                if chars.next_if_eq(&'/').is_some() {
                    while chars.next_if(|c| c != &'\n').is_some() {}
                    continue;
                }
                Token::Slash
            }
            ' ' | '\t' => {
                continue;
            }
            '\n' => {
                line_count += 1;
                continue;
            }
            '"' => {
                let mut ended = false;
                let literal = chars
                    .by_ref()
                    .inspect(|c| {
                        if c == &'"' {
                            ended = true;
                        }
                    })
                    .take_while(|c| c != &'"')
                    .fold(String::new(), |mut acc, c| {
                        acc.push(c);
                        acc
                    });
                if ended {
                    Token::String(literal)
                } else {
                    eprintln!("[line {line_count}] Error: Unterminated string.");
                    lexical_error = true;
                    continue;
                }
            }
            '0'..='9' => {
                let mut number_str = String::from(c);

                let mut first_dot = false;
                while let Some(c) = chars.peek() {
                    if c == &'.' && first_dot {
                        break;
                    }
                    if c == &'.' {
                        first_dot = true;
                        number_str.push(*c);
                        chars.next();
                        continue;
                    }

                    if !c.is_ascii_digit() {
                        break;
                    }
                    number_str.push(*c);
                    chars.next();
                }

                let number: f64 = number_str.parse().unwrap();
                Token::Number(number, number_str)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(c);
                // We ignore reserved word for now
                // What we really want is a take while that doesn't consume the last character.
                while let Some(c) =
                    chars.next_if(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' ))
                {
                    identifier.push(c);
                }
                match identifier.as_str() {
                    "and" => Token::And,
                    "class" => Token::Class,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "for" => Token::For,
                    "fun" => Token::Fun,
                    "if" => Token::If,
                    "nil" => Token::Nil,
                    "or" => Token::Or,
                    "return" => Token::Return,
                    "super" => Token::Super,
                    "this" => Token::This,
                    "true" => Token::True,
                    "var" => Token::Var,
                    "while" => Token::While,
                    "print" => Token::Print,
                    ident => Token::Identifier(ident.into()),
                }
            }
            c => {
                eprintln!("[line {line_count}] Error: Unexpected character: {c}");
                lexical_error = true;
                continue;
            }
        };
        tokens.push(token);
    }
    if lexical_error {
        Err(tokens)
    } else {
        Ok(tokens)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    EqualEqual,
    Equal,
    BangEqual,
    Bang,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    Slash,
    String(String),
    Number(f64, String),
    Identifier(String),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Print,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Less => write!(f, "LESS < null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::String(literal) => write!(f, "STRING \"{literal}\" {literal}"),
            Token::Number(number, number_str) => {
                if number.fract() == 0.0 {
                    write!(f, "NUMBER {number_str} {number}.0")
                } else {
                    write!(f, "NUMBER {number_str} {number}")
                }
            }
            Token::Identifier(ident) => write!(f, "IDENTIFIER {ident} null"),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::For => write!(f, "FOR for null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
            Token::Print => write!(f, "PRINT print null"),
        }
    }
}
