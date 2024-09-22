use core::fmt;
use std::env;
use std::fs;

use lex::tokenize;
use lex::Token;
mod lex;

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
            let token_tree = parse_token(&tokens);
            println!("{token_tree}");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn parse_token<'de>(tokens: &[Token<'de>]) -> TokenTree<'de> {
    eprintln!("**** parse tokens ****");
    dbg!(tokens);
    let mut tokens_iter = tokens.iter().enumerate().peekable();
    let mut token_tree = TokenTree::Primary(Primary::Nil);
    while let Some((i, token)) = tokens_iter.next() {
        dbg!(i);
        token_tree = match token {
            Token::Nil => TokenTree::Primary(Primary::Nil),
            Token::True => TokenTree::Primary(Primary::True),
            Token::False => TokenTree::Primary(Primary::False),
            Token::Number(n, _) => TokenTree::Primary(Primary::Number(*n)),
            Token::String(s) => TokenTree::Primary(Primary::String(s)),
            Token::LeftParen => {
                let token = parse_token(&tokens[i + 1..tokens.len()]);
                dbg!(&token);
                while tokens_iter.next().is_some() {}

                TokenTree::Primary(Primary::Group(Box::new(token)))
            }
            Token::RightParen => {
                eprintln!("Encoutered right paren");
                continue;
            }
            _ => panic!("unhandled token"),
        };
    }
    token_tree
}

#[derive(Debug, PartialEq)]
enum TokenTree<'de> {
    Primary(Primary<'de>),
}

impl fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Primary(prim) => write!(f, "{prim}"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Primary<'de> {
    String(&'de str),
    Number(f64),
    True,
    False,
    Nil,
    Group(Box<TokenTree<'de>>),
}

impl fmt::Display for Primary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primary::String(s) => write!(f, "{s}"),
            Primary::Number(n) => write!(f, "{n:?}"),
            Primary::True => write!(f, "true"),
            Primary::False => write!(f, "false"),
            Primary::Nil => write!(f, "nil"),
            Primary::Group(tt) => write!(f, "(group {tt})"),
        }
    }
}
