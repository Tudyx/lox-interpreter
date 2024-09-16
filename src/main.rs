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

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                tokenize(&file_contents);
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(file_content: &str) {
    let mut lexical_error = false;
    let mut line_count = 1;

    let mut chars = file_content.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            ',' => println!("COMMA , null"),
            '.' => println!("DOT . null"),
            '-' => println!("MINUS - null"),
            '+' => println!("PLUS + null"),
            ';' => println!("SEMICOLON ; null"),
            '*' => println!("STAR * null"),
            '=' => {
                if chars.next_if_eq(&'=').is_some() {
                    println!("EQUAL_EQUAL == null");
                } else {
                    println!("EQUAL = null")
                }
            }
            '!' => {
                if chars.next_if_eq(&'=').is_some() {
                    println!("BANG_EQUAL != null");
                } else {
                    println!("BANG ! null")
                }
            }
            '<' => {
                if chars.next_if_eq(&'=').is_some() {
                    println!("LESS_EQUAL <= null");
                } else {
                    println!("LESS < null")
                }
            }
            '>' => {
                if chars.next_if_eq(&'=').is_some() {
                    println!("GREATER_EQUAL >= null");
                } else {
                    println!("GREATER > null")
                }
            }
            '/' => {
                // We ignore the rest of the line
                if chars.next_if_eq(&'/').is_some() {
                    while chars.by_ref().next().is_some() {
                        if chars.peek().is_some_and(|c| c == &'\n') {
                            break;
                        }
                    }
                } else {
                    println!("SLASH / null")
                }
            }
            ' ' | '\t' => {}
            '\n' => line_count += 1,
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
                    println!("STRING \"{literal}\" {literal}");
                } else {
                    eprintln!("[line {line_count}] Error: Unterminated string.");
                    lexical_error = true;
                }
            }
            c if c.is_ascii_digit() => {
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

                if number.fract() == 0.0 {
                    println!("NUMBER {number_str} {number}.0");
                } else {
                    println!("NUMBER {number_str} {number}");
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(c);
                // We ignore reserved word for now
                // What we really want is a take while that doesn't consume the last character.
                const AND: &str = "and";
                while let Some(c) =
                    chars.next_if(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' ))
                {
                    identifier.push(c);
                }
                if identifier == AND {
                    println!("AND and null");
                } else {
                    println!("IDENTIFIER {identifier} null");
                }
            }
            c => {
                eprintln!("[line {line_count}] Error: Unexpected character: {c}");
                lexical_error = true;
            }
        }
    }
    println!("EOF  null");
    if lexical_error {
        std::process::exit(65);
    }
}
