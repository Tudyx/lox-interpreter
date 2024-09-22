use core::fmt;
use core::panic;
use std::env;
use std::fs;
use std::iter::Peekable;

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
            let tokens = &mut tokens.into_iter().peekable();
            let token_tree = parse_tokens(tokens);
            println!("{token_tree}");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
fn parse_tokens<'de>(tokens: &mut Peekable<impl Iterator<Item = Token<'de>>>) -> TokenTree<'de> {
    eprintln!("**** parse tokens ****");
    if let Some(token) = tokens.next() {
        return match token {
            Token::Nil => TokenTree::Primary(Primary::Nil),
            Token::True => TokenTree::Primary(Primary::True),
            Token::False => TokenTree::Primary(Primary::False),
            Token::Number(n, _) => TokenTree::Primary(Primary::Number(n)),
            Token::String(s) => TokenTree::Primary(Primary::String(s)),
            Token::LeftParen => {
                let token_tree = TokenTree::Primary(Primary::Group(Box::new(parse_tokens(tokens))));
                if !tokens
                    .next()
                    .is_some_and(|token| token == Token::RightParen)
                {
                    panic!("Miss Right Paren");
                }
                token_tree
            }
            // prefix operator
            Token::Minus => TokenTree::Unary(Unary::Minus(Box::new(parse_tokens(tokens)))),
            Token::Bang => TokenTree::Unary(Unary::Bang(Box::new(parse_tokens(tokens)))),
            // infix operator
            Token::Star => {
                todo!()
            }
            Token::Slash => {
                todo!()
            }
            _ => panic!("unhandled token"),
        };
    }
    TokenTree::Primary(Primary::Nil)
}

#[derive(Debug, PartialEq)]
enum TokenTree<'de> {
    Primary(Primary<'de>),
    Unary(Unary<'de>),
    Factor(Factor<'de>),
}

impl fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Primary(prim) => write!(f, "{prim}"),
            TokenTree::Unary(unary) => write!(f, "{unary}"),
            TokenTree::Factor(factor) => write!(f, "{factor}"),
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
#[derive(Debug, PartialEq)]
enum Unary<'de> {
    Bang(Box<TokenTree<'de>>),
    Minus(Box<TokenTree<'de>>),
}

impl fmt::Display for Unary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unary::Bang(tt) => write!(f, "(! {tt})"),
            Unary::Minus(tt) => write!(f, "(- {tt})"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Factor<'de> {
    Slash(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    Star(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
}

impl fmt::Display for Factor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Factor::Slash(left, right) => write!(f, "(/ {left} {right})"),
            Factor::Star(left, right) => write!(f, "(* {left} {right})"),
        }
    }
}
