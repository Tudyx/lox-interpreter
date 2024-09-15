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
                // TODO: test me
                // let mut ended = false;
                // let literal = chars
                //     .by_ref()
                //     .take_while(|c| {
                //         if c == &'"' {
                //             ended = true;
                //         }
                //         c != &'"'
                //     })
                //     .fold(String::new(), |mut acc, c| {
                //         acc.push(c);
                //         acc
                //     });

                let mut literal = String::new();
                let mut ended = false;
                for c in chars.by_ref() {
                    if c == '"' {
                        ended = true;
                        break;
                    }
                    literal.push(c);
                }
                if ended {
                    println!("STRING \"{literal}\" {literal}");
                } else {
                    eprintln!("[line {line_count}] Error: Unterminated string.");
                    lexical_error = true;
                }
            }
            c if c.is_ascii_digit() => {
                let number_str = chars
                    .by_ref()
                    .take_while(|c| c.is_ascii_digit() || c == &'.')
                    .fold(String::from(c), |mut acc, c| {
                        acc.push(c);
                        acc
                    });

                let number: f64 = number_str.parse().unwrap();

                if number.fract() == 0.0 {
                    println!("NUMBER {number_str} {number}.0");
                } else {
                    println!("NUMBER {number_str} {number}");
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
