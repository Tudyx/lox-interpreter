mod interpreter;
mod lex;
mod parse;

use crate::{
    interpreter::Interpreter,
    lex::Lexer,
    parse::{parse_expr, parse_statements},
};
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
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(65)
                }
            });
            let tokens = &mut tokens.into_iter().peekable();
            let Ok(token_tree) = parse_expr(tokens, 0) else {
                std::process::exit(65);
            };
            println!("{token_tree}");
        }
        "evaluate" => {
            let tokens = Lexer::new(&file_contents).map(|token| match token {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(65)
                }
            });
            let tokens = &mut tokens.into_iter().peekable();

            let Ok(token_tree) = parse_expr(tokens, 0) else {
                std::process::exit(65);
            };

            let mut interpreter = Interpreter::new();

            match interpreter.evaluate_expr(token_tree) {
                Ok(value) => println!("{value}"),
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(70);
                }
            };
        }
        "run" => {
            let tokens = Lexer::new(&file_contents).map(|token| match token {
                Ok(token) => token,
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(65)
                }
            });
            let tokens = &mut tokens.into_iter().peekable();
            let token_tree = match parse_statements(tokens) {
                Ok(token_tree) => token_tree,
                Err(err) => {
                    eprintln!("Failed to parse the statements: {err}");
                    std::process::exit(65)
                }
            };
            // eprintln!("Find {} statement", token_tree.len());
            let mut interpreter = Interpreter::new();
            if let Err(err) = interpreter.evaluate(token_tree) {
                eprintln!("{err}");
                std::process::exit(70);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
