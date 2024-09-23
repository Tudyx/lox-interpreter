use std::fmt;

// struct Lexer<'de> {
//     file_content: &'de str,
// }

// impl<'de> Iterator for Lexer<'de> {
//     type Item = Result<Token<'de>, ()>;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

pub fn tokenize(file_content: &str) -> Result<Vec<Token>, Vec<Token>> {
    let mut lexical_error = false;
    let mut line_count = 1;
    let mut tokens = Vec::new();

    let mut chars = file_content.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
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
                if chars.next_if(|(_, c)| c == &'=').is_some() {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if chars.next_if(|(_, c)| c == &'=').is_some() {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if chars.next_if(|(_, c)| c == &'=').is_some() {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if chars.next_if(|(_, c)| c == &'=').is_some() {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '/' => {
                // We ignore the rest of the line
                if chars.next_if(|(_, c)| c == &'/').is_some() {
                    while chars.next_if(|(_, c)| c != &'\n').is_some() {}
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
            '"' => match chars.position(|(_, c)| c == '"') {
                Some(end) => Token::String(&file_content[i + 1..=i + end]),
                None => {
                    eprintln!("[line {line_count}] Error: Unterminated string.");
                    lexical_error = true;
                    continue;
                }
            },
            '0'..='9' => {
                let mut first_dot = false;
                let mut end = i;
                while let Some((_, c)) = chars.peek() {
                    if c == &'.' && first_dot {
                        break;
                    }
                    if c == &'.' {
                        first_dot = true;
                        end += 1;
                        chars.next();
                        continue;
                    }

                    if !c.is_ascii_digit() {
                        break;
                    }
                    end += 1;
                    chars.next();
                }

                let number_str = &file_content[i..=end];
                let number: f64 = number_str.parse().unwrap();
                Token::Number(number, number_str)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end = i;
                while chars
                    .next_if(|(_, c)| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' ))
                    .is_some()
                {
                    end += 1;
                }
                let identifier = &file_content[i..=end];
                match identifier {
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
                    ident => Token::Identifier(ident),
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
pub enum Token<'de> {
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
    String(&'de str),
    Number(f64, &'de str),
    Identifier(&'de str),
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

impl fmt::Display for Token<'_> {
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
            Token::Number(number, number_str) => write!(f, "NUMBER {number_str} {number:?}"),
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
