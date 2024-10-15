use std::{borrow::Cow, collections::HashMap, fmt};

use crate::parse::{
    Comparison, Equality, ExpressionTree, Factor, Primary, StatementTree, Term, Unary,
};

pub struct Interpreter<'de> {
    /// Map variable identitifer and their value.
    variables: HashMap<&'de str, Value<'de>>,
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
                        self.variables.insert(ident, Value::Nil);
                    }
                }
                StatementTree::Block(statements) => self.evaluate(statements)?,
            };
        }
        Ok(())
    }

    pub fn evaluate_expr(
        &mut self,
        token_tree: ExpressionTree<'de>,
    ) -> Result<Value<'de>, EvaluationError<'de>> {
        Ok(match token_tree {
            ExpressionTree::Primary(primary) => match primary {
                Primary::String(string) => Value::String(Cow::Borrowed(string)),
                Primary::Number(number) => Value::Number(number),
                Primary::True => Value::Boolean(true),
                Primary::False => Value::Boolean(false),
                Primary::Nil => Value::Nil,
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
                        Value::Boolean(boolean) => match boolean {
                            true => Value::Boolean(false),
                            false => Value::Boolean(true),
                        },
                        Value::Number(_) | Value::String(_) => Value::Boolean(false),
                        Value::Nil => Value::Boolean(true),
                    }
                }
                Unary::Minus(token_tree) => {
                    let value_tmp = self.evaluate_expr(*token_tree)?;
                    let value = value_tmp.as_number()?;

                    Value::Number(-value)
                }
            },
            ExpressionTree::Factor(factor) => match factor {
                Factor::Slash(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Number(lhs / rhs)
                }
                Factor::Star(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Number(lhs * rhs)
                }
            },
            ExpressionTree::Term(term) => match term {
                Term::Minus(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Number(lhs - rhs)
                }
                Term::Plus(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
                        (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + rhs),
                        _ => return Err(EvaluationError::WrongPlusOperands),
                    }
                }
            },
            ExpressionTree::Comparison(comparison) => match comparison {
                Comparison::Less(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Boolean(lhs < rhs)
                }
                Comparison::LessEqual(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Boolean(lhs <= rhs)
                }
                Comparison::Greater(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Boolean(lhs > rhs)
                }
                Comparison::GreaterEqual(lhs, rhs) => {
                    let lhs = self.evaluate_expr(*lhs)?.as_number()?;
                    let rhs = self.evaluate_expr(*rhs)?.as_number()?;
                    Value::Boolean(lhs >= rhs)
                }
            },
            ExpressionTree::Equality(equality) => match equality {
                Equality::EqualEqual(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Value::Boolean(lhs), Value::Boolean(rhs)) => Value::Boolean(lhs == rhs),
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs == rhs),
                        (Value::String(lhs), Value::String(rhs)) => Value::Boolean(lhs == rhs),
                        (Value::Nil, Value::Nil) => Value::Boolean(true),
                        _ => Value::Boolean(false),
                    }
                }
                Equality::BangEqual(lhs, rhs) => {
                    match (self.evaluate_expr(*lhs)?, self.evaluate_expr(*rhs)?) {
                        (Value::Boolean(lhs), Value::Boolean(rhs)) => Value::Boolean(lhs != rhs),
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs != rhs),
                        (Value::String(lhs), Value::String(rhs)) => Value::Boolean(lhs != rhs),
                        (Value::Nil, Value::Nil) => Value::Boolean(true),
                        _ => Value::Boolean(false),
                    }
                }
            },
            ExpressionTree::Assignment(ident, expr) => {
                if !self.variables.contains_key(ident) {
                    return Err(EvaluationError::UndeclaredVariable(ident));
                }

                let value = self.evaluate_expr(*expr)?;
                self.variables.insert(ident, value.clone());
                value
            }
        })
    }
}

// An instance of this type is a value.
#[derive(Clone)]
pub enum Value<'de> {
    Boolean(bool),
    Number(f64),
    String(Cow<'de, str>),
    Nil,
}

// We use explicit lifetime here because otherwise lifetime elision
// will bind the lifetime of the return type to `self` but it must be bound to
// the file content lifetime. (The one in the string variant)
impl<'de> Value<'de> {
    fn as_number(&self) -> Result<f64, EvaluationError<'de>> {
        if let Value::Number(value) = &self {
            Ok(*value)
        } else {
            Err(EvaluationError::ExpectedNumber)
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(boolean) => write!(f, "{boolean}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "{string}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub enum EvaluationError<'de> {
    ExpectedNumber,
    UndeclaredVariable(&'de str),
    UndefinedVariable(&'de str),
    WrongPlusOperands,
}

impl<'de> std::error::Error for EvaluationError<'de> {}

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
            EvaluationError::UndeclaredVariable(ident) => {
                write!(f, "Undeclared variable '{ident}'.")
            }
        }
    }
}
