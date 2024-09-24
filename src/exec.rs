use std::fmt;

use crate::parse::{Factor, Primary, Term, TokenTree, Unary};

pub fn evaluate_expr(token_tree: TokenTree<'_>) -> Result<Ty<'_>, EvaluationError> {
    Ok(match token_tree {
        TokenTree::Primary(primary) => match primary {
            Primary::String(string) => Ty::String(string),
            Primary::Number(number) => Ty::Number(number),
            Primary::True => Ty::Boolean(true),
            Primary::False => Ty::Boolean(false),
            Primary::Nil => Ty::Nil,
            Primary::Group(token_tree) => evaluate_expr(*token_tree)?,
        },
        TokenTree::Unary(unary) => match unary {
            Unary::Bang(token_tree) => {
                let value = evaluate_expr(*token_tree)?;
                match value {
                    Ty::Boolean(boolean) => match boolean {
                        true => Ty::Boolean(false),
                        false => Ty::Boolean(true),
                    },
                    Ty::Number(_) | Ty::String(_) => Ty::Boolean(false),
                    Ty::Nil => Ty::Boolean(true),
                }
            }
            Unary::Minus(token_tree) => {
                let value = evaluate_expr(*token_tree)?.as_number()?;
                Ty::Number(-value)
            }
        },
        TokenTree::Factor(factor) => match factor {
            Factor::Slash(lhs, rhs) => {
                let lhs = evaluate_expr(*lhs)?.as_number()?;
                let rhs = evaluate_expr(*rhs)?.as_number()?;
                Ty::Number(lhs / rhs)
            }
            Factor::Star(lhs, rhs) => {
                let lhs = evaluate_expr(*lhs)?.as_number()?;
                let rhs = evaluate_expr(*rhs)?.as_number()?;
                Ty::Number(lhs * rhs)
            }
        },
        TokenTree::Term(term) => match term {
            Term::Minus(lhs, rhs) => {
                let lhs = evaluate_expr(*lhs)?.as_number()?;
                let rhs = evaluate_expr(*rhs)?.as_number()?;
                Ty::Number(lhs - rhs)
            }
            Term::Plus(lhs, rhs) => {
                let lhs = evaluate_expr(*lhs)?.as_number()?;
                let rhs = evaluate_expr(*rhs)?.as_number()?;
                Ty::Number(lhs + rhs)
            }
        },
        TokenTree::Comparison(_) => todo!(),
        TokenTree::Equality(_) => todo!(),
    })
}

// An instance of this type is a value.
pub enum Ty<'de> {
    Boolean(bool),
    Number(f64),
    String(&'de str),
    Nil,
}

impl Ty<'_> {
    fn as_number(&self) -> Result<f64, EvaluationError> {
        if let Ty::Number(value) = &self {
            Ok(*value)
        } else {
            Err(EvaluationError::WrongType)
        }
    }
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

#[derive(Debug)]
pub enum EvaluationError {
    WrongType,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::WrongType => write!(f, "wrong type"),
        }
    }
}
