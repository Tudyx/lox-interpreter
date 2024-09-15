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
    for (i, c) in file_content.chars().enumerate() {
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
                // We assume that the text contains only ASCII charactere
                if file_content
                    .as_bytes()
                    .get(i + 1)
                    .is_some_and(|octet| *octet == b'=')
                {
                    println!("EQUAL_EQUAL == null");
                } else {
                    println!("EQUAL = null")
                }
            }
            c => {
                eprintln!("[line 1] Error: Unexpected character: {c}");
                lexical_error = true;
            }
        }
    }
    println!("EOF  null");
    if lexical_error {
        std::process::exit(65);
    }
}
