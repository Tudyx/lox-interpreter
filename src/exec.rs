use std::fmt;

use crate::parse::{Factor, Primary, TokenTree};

pub fn evaluate_expr<'de>(token_tree: TokenTree<'de>) -> Ty<'de> {
    match token_tree {
        TokenTree::Primary(primary) => match primary {
            Primary::String(string) => Ty::String(string),
            Primary::Number(number) => Ty::Number(number),
            Primary::True => Ty::Boolean(true),
            Primary::False => Ty::Boolean(false),
            Primary::Nil => Ty::Nil,
            Primary::Group(token_tree) => evaluate_expr(*token_tree),
        },
        TokenTree::Unary(_) => todo!(),
        TokenTree::Factor(factor) => match factor {
            Factor::Slash(lhs, rhs) => todo!(),
            Factor::Star(lhs, rhs) => todo!(),
        },
        TokenTree::Term(_) => todo!(),
        TokenTree::Comparison(_) => todo!(),
        TokenTree::Equality(_) => todo!(),
    }
}

// An instance of this type is a value.
pub enum Ty<'de> {
    Boolean(bool),
    Number(f64),
    String(&'de str),
    Nil,
}

impl fmt::Display for Ty<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Boolean(boolean) => write!(f, "{boolean}"),
            Ty::Number(number) => write!(f, "{number}"),
            Ty::String(string) => write!(f, "{string}"),
            Ty::Nil => write!(f, "nil"),
        }
    }
}
