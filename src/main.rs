mod lex;
mod parse;

use crate::parse::parse_tokens;
use lex::tokenize;
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
            let Ok(tokens) = tokenize(&file_contents) else {
                std::process::exit(65);
            };
            let tokens = &mut tokens.into_iter().peekable();
            let Ok(token_tree) = parse_tokens(tokens, 0) else {
                std::process::exit(65);
            };
            println!("{token_tree}");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
