mod exec;
mod lex;
mod parse;

use crate::{exec::evaluate_expr, lex::Lexer, parse::parse_tokens};
use std::{env, fs};

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
            let mut lexical_error = false;
            if !file_contents.is_empty() {
                let lexer = Lexer::new(&file_contents);
                for token in lexer {
                    match token {
                        Ok(token) => println!("{token}"),
                        Err(err) => {
                            eprintln!("{err}");
                            lexical_error = true
                        }
                    }
                }
            }
            println!("EOF  null");
            if lexical_error {
                std::process::exit(65);
            }
        }
        "parse" => {
            let tokens = Lexer::new(&file_contents).map(|token| match token {
                Ok(token) => token,
                Err(_) => std::process::exit(65),
            });
            let tokens = &mut tokens.into_iter().peekable();
            let Ok(token_tree) = parse_tokens(tokens, 0) else {
                std::process::exit(65);
            };
            println!("{token_tree}");
        }
        "evaluate" => {
            let tokens = Lexer::new(&file_contents).map(|token| match token {
                Ok(token) => token,
                Err(_) => std::process::exit(65),
            });
            let tokens = &mut tokens.into_iter().peekable();
            let Ok(token_tree) = parse_tokens(tokens, 0) else {
                std::process::exit(65);
            };
            match evaluate_expr(token_tree) {
                Ok(value) => println!("{value}"),
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(65);
                }
            };
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
