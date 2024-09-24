use std::{fmt, iter::Peekable};

use crate::lex::Token;

pub fn parse_tokens<'de>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'de>>>,
    min_bp: u8,
) -> Result<TokenTree<'de>, ()> {
    let mut lhs = if let Some(token) = tokens.next() {
        match token {
            Token::Nil => TokenTree::Primary(Primary::Nil),
            Token::True => TokenTree::Primary(Primary::True),
            Token::False => TokenTree::Primary(Primary::False),
            Token::Number(n, _) => TokenTree::Primary(Primary::Number(n)),
            Token::String(s) => TokenTree::Primary(Primary::String(s)),
            Token::LeftParen => {
                let token_tree =
                    TokenTree::Primary(Primary::Group(Box::new(parse_tokens(tokens, 0)?)));
                if !tokens
                    .next()
                    .is_some_and(|token| token == Token::RightParen)
                {
                    panic!("Miss Right Paren");
                }
                token_tree
            }
            // prefix operator (Unary)
            Token::Minus => TokenTree::Unary(Unary::Minus(Box::new(parse_tokens(tokens, 5)?))),
            Token::Bang => TokenTree::Unary(Unary::Bang(Box::new(parse_tokens(tokens, 5)?))),
            _ => return Err(()),
        }
    } else {
        TokenTree::Primary(Primary::Nil)
    };

    // Here we don't passe the rest of the token to the recursive call,
    // so we must loop ourself
    while let Some(next_token) = tokens.peek() {
        match next_token {
            Token::Star => {
                let bp = 4;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                // Here we want to pass the next items until we encounter something that have the same level of
                // precedence that the Star. If it's lower, for instance a +, we stop
                // let rhs = parse_tokens(&mut std::iter::once(tokens.next().unwrap()).peekable());
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Factor(Factor::Star(Box::new(lhs), Box::new(rhs)));
            }
            Token::Slash => {
                let bp = 4;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                // let rhs = parse_tokens(&mut std::iter::once(tokens.next().unwrap()).peekable());
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Factor(Factor::Slash(Box::new(lhs), Box::new(rhs)));
            }
            Token::Plus => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Term(Term::Plus(Box::new(lhs), Box::new(rhs)));
            }
            Token::Minus => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Term(Term::Minus(Box::new(lhs), Box::new(rhs)));
            }
            Token::Less => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Comparison(Comparison::Less(Box::new(lhs), Box::new(rhs)));
            }
            Token::LessEqual => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Comparison(Comparison::LessEqual(Box::new(lhs), Box::new(rhs)));
            }
            Token::Greater => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }

                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Comparison(Comparison::Greater(Box::new(lhs), Box::new(rhs)));
            }
            Token::GreaterEqual => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Comparison(Comparison::GreaterEqual(Box::new(lhs), Box::new(rhs)));
            }

            Token::EqualEqual => {
                let bp = 1;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Equality(Equality::EqualEqual(Box::new(lhs), Box::new(rhs)));
            }
            Token::BangEqual => {
                let bp = 1;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_tokens(tokens, bp)?;
                lhs = TokenTree::Equality(Equality::BangEqual(Box::new(lhs), Box::new(rhs)));
            }
            _ => {
                break;
            }
        }
    }

    Ok(lhs)
}
// We only have left associativity (exept for prefix operator) so we can use only one binding power number

// fn infix_binding_power() -> (u8, u8) {}

#[derive(Debug, PartialEq)]
pub enum TokenTree<'de> {
    Primary(Primary<'de>),
    Unary(Unary<'de>),
    Factor(Factor<'de>),
    Term(Term<'de>),
    Comparison(Comparison<'de>),
    Equality(Equality<'de>),
}

impl fmt::Display for TokenTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenTree::Primary(prim) => write!(f, "{prim}"),
            TokenTree::Unary(unary) => write!(f, "{unary}"),
            TokenTree::Factor(factor) => write!(f, "{factor}"),
            TokenTree::Term(term) => write!(f, "{term}"),
            TokenTree::Comparison(comparison) => write!(f, "{comparison}"),
            TokenTree::Equality(equality) => write!(f, "{equality}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Primary<'de> {
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
pub enum Unary<'de> {
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
pub enum Factor<'de> {
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

#[derive(Debug, PartialEq)]
pub enum Term<'de> {
    Minus(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    Plus(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
}

impl fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Minus(left, right) => write!(f, "(- {left} {right})"),
            Term::Plus(left, right) => write!(f, "(+ {left} {right})"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Comparison<'de> {
    Less(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    LessEqual(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    Greater(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    GreaterEqual(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
}

impl fmt::Display for Comparison<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Comparison::Less(left, right) => write!(f, "(< {left} {right})"),
            Comparison::LessEqual(left, right) => write!(f, "(<= {left} {right})"),
            Comparison::Greater(left, right) => write!(f, "(> {left} {right})"),
            Comparison::GreaterEqual(left, right) => write!(f, "(>= {left} {right})"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Equality<'de> {
    EqualEqual(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
    BangEqual(Box<TokenTree<'de>>, Box<TokenTree<'de>>),
}

impl fmt::Display for Equality<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Equality::EqualEqual(left, right) => write!(f, "(== {left} {right})"),
            Equality::BangEqual(left, right) => write!(f, "(!= {left} {right})"),
        }
    }
}
