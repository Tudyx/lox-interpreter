use std::{borrow::Cow, collections::HashMap, fmt};

use crate::parse::{
    Comparison, Equality, ExpressionTree, Factor, Primary, StatementTree, Term, Unary,
};

pub struct Interpreter<'de> {
    variables: HashMap<&'de str, Ty<'de>>,
}

impl<'de> Interpreter<'de> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    pub fn evaluate(
        &mut self,
        token_tree: Vec<StatementTree<'de>>,
    ) -> Result<(), EvaluationError<'de>> {
        for statement in token_tree {
            match statement {
                StatementTree::Print(expr) => {
                    let value = self.evaluate_expr(expr)?;
                    println!("{value}");
                }
                StatementTree::Expr(expr) => {
                    // Expression statement is for expression
                    // that have side effects.
                    let _ = self.evaluate_expr(expr)?;
                }
                StatementTree::VarDeclaration { ident, expr } => {
                    if let Some(expr) = expr {
                        let value = self.evaluate_expr(expr)?;
                        self.variables.insert(ident, value);
                    } else {
                        self.variables.insert(ident, Ty::Nil);
                    }
                }
            };
        }
        Ok(())
    }

    pub fn evaluate_expr(
        &self,
        token_tree: ExpressionTree<'de>,
    ) -> Result<Ty<'de>, EvaluationError<'de>> {
        Ok(match token_tree {
            ExpressionTree::Primary(primary) => match primary {
                Primary::String(string) => Ty::String(Cow::Borrowed(string)),
                Primary::Number(number) => Ty::Number(number),
                Primary::True => Ty::Boolean(true),
                Primary::False => Ty::Boolean(false),
                Primary::Nil => Ty::Nil,
                Primary::Group(token_tree) => self.evaluate_expr(*token_tree)?,
                Primary::Identifier(ident) => self
                    .variables
                    .get(ident)
                    .ok_or(EvaluationError::UndefinedVariable(ident))?
                    .clone(),
            },
            ExpressionTree::Unary(unary) => match unary {
                Unary::Bang(token_tree) => {
                    let value = self.evaluate_expr(*token_tree)?;
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
                    let value_tmp = self.evaluate_expr(*token_tree)?;
                    let value = value_tmp.as_number()?;

                    Ty::Number(-value)
                }
            },
            ExpressionTree::Factor(factor) => match factor {
                Factor::Slash(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Number(lhs / rhs)
                }
                Factor::Star(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Number(lhs * rhs)
                }
            },
            ExpressionTree::Term(term) => match term {
                Term::Minus(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Number(lhs - rhs)
                }
                Term::Plus(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Ty::Number(lhs), Ty::Number(rhs)) => Ty::Number(lhs + rhs),
                        (Ty::String(lhs), Ty::String(rhs)) => Ty::String(lhs + rhs),
                        _ => return Err(EvaluationError::WrongPlusOperands),
                    }
                }
            },
            ExpressionTree::Comparison(comparison) => match comparison {
                Comparison::Less(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Boolean(lhs < rhs)
                }
                Comparison::LessEqual(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Boolean(lhs <= rhs)
                }
                Comparison::Greater(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Boolean(lhs > rhs)
                }
                Comparison::GreaterEqual(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Ty::Boolean(lhs >= rhs)
                }
            },
            ExpressionTree::Equality(equality) => match equality {
                Equality::EqualEqual(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Ty::Boolean(lhs), Ty::Boolean(rhs)) => Ty::Boolean(lhs == rhs),
                        (Ty::Number(lhs), Ty::Number(rhs)) => Ty::Boolean(lhs == rhs),
                        (Ty::String(lhs), Ty::String(rhs)) => Ty::Boolean(lhs == rhs),
                        (Ty::Nil, Ty::Nil) => Ty::Boolean(true),
                        _ => Ty::Boolean(false),
                    }
                }
                Equality::BangEqual(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Ty::Boolean(lhs), Ty::Boolean(rhs)) => Ty::Boolean(lhs != rhs),
                        (Ty::Number(lhs), Ty::Number(rhs)) => Ty::Boolean(lhs != rhs),
                        (Ty::String(lhs), Ty::String(rhs)) => Ty::Boolean(lhs != rhs),
                        (Ty::Nil, Ty::Nil) => Ty::Boolean(true),
                        _ => Ty::Boolean(false),
                    }
                }
            },
        })
    }
}

// An instance of this type is a value.
#[derive(Clone)]
pub enum Ty<'de> {
    Boolean(bool),
    Number(f64),
    String(Cow<'de, str>),
    Nil,
}

// We use explicit lifetime here because otherwise lifetime elision
// will bind the lifetime of the return type to `self` but it must be bound to
// the file content lifetime.
impl<'de> Ty<'de> {
    fn as_number(&self) -> Result<f64, EvaluationError<'de>> {
        if let Ty::Number(value) = &self {
            Ok(*value)
        } else {
            Err(EvaluationError::ExpectedNumber)
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
pub enum EvaluationError<'de> {
    ExpectedNumber,
    WrongPlusOperands,
    UndefinedVariable(&'de str),
}

impl fmt::Display for EvaluationError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::ExpectedNumber => {
                let var_name = write!(f, "Operand must be a number.");
                var_name
            }
            EvaluationError::WrongPlusOperands => {
                write!(f, "Operands must be two numbers or two strings.")
            }
            EvaluationError::UndefinedVariable(ident) => write!(f, "Undefined variable '{ident}'."),
        }
    }
}
