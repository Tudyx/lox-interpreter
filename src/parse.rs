use std::{fmt, iter::Peekable};

use crate::lex::Token;

// As we only want a single token lookahead, `Peekable` is all we need.
//
// Lifetime elision will put the wrong lifetime to the return time so we
// must be explicit.
pub fn parse_statements<'de>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'de>>>,
) -> Result<Vec<StatementTree<'de>>, ParseExpressionError<'de>> {
    let mut statements = Vec::new();
    while let Some(statement) = parse_statement(tokens)? {
        statements.push(statement);
    }

    Ok(statements)
}

pub fn parse_statement<'de>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'de>>>,
) -> Result<Option<StatementTree<'de>>, ParseExpressionError<'de>> {
    let Some(token) = tokens.peek() else {
        return Ok(None);
    };
    // A program is just 0 or more statements
    let statement = match token {
        Token::Print => {
            tokens.next();
            let expr = parse_expr(tokens, 0)?;
            if let Some(token) = tokens.next() {
                if token != Token::Semicolon {
                    panic!("Expected semicolon got '{token}'");
                }
            }
            StatementTree::Print(expr)
        }
        Token::Var => {
            tokens.next();
            let Some(Token::Identifier(ident)) = tokens.next() else {
                panic!("Expected identifier");
            };
            let expr = if tokens.next_if_eq(&Token::Equal).is_some() {
                Some(parse_expr(tokens, 0)?)
            } else {
                None
            };

            if let Some(token) = tokens.next() {
                if token != Token::Semicolon {
                    panic!("Expected semicolon got '{token}'");
                }
            }
            StatementTree::VarDeclaration { ident, expr }
        }

        Token::LeftBrace => {
            tokens.next();
            let mut block_statements = Vec::new();

            while let Some(statement) = parse_statement(tokens)? {
                block_statements.push(statement);
                if tokens
                    .peek()
                    .is_some_and(|token| token == &Token::RightBrace)
                {
                    break;
                }
            }

            if !matches!(tokens.next(), Some(Token::RightBrace)) {
                return Err(ParseExpressionError::MissingRightBrace);
            }
            StatementTree::Block(block_statements)
        }
        Token::RightBrace => {
            unreachable!();
        }
        _ => {
            let expr = parse_expr(tokens, 0)?;
            if let Some(token) = tokens.next() {
                if token != Token::Semicolon {
                    panic!("Expected semicolon got '{token}'");
                }
            }
            StatementTree::Expr(expr)
        }
    };
    Ok(Some(statement))
}

pub enum StatementTree<'de> {
    /// Print statement.
    Print(ExpressionTree<'de>),
    /// Expression statement, for expression that have side effect.
    Expr(ExpressionTree<'de>),
    /// Block statement. In Lox they don't produce value, like in
    /// Rust where block are expression.
    Block(Vec<StatementTree<'de>>),
    VarDeclaration {
        ident: &'de str,
        expr: Option<ExpressionTree<'de>>,
    },
}

// Pratt parser
pub fn parse_expr<'de>(
    tokens: &mut Peekable<impl Iterator<Item = Token<'de>>>,
    min_bp: u8,
) -> Result<ExpressionTree<'de>, ParseExpressionError<'de>> {
    let mut lhs = if let Some(token) = tokens.next() {
        match token {
            Token::Nil => ExpressionTree::Primary(Primary::Nil),
            Token::True => ExpressionTree::Primary(Primary::True),
            Token::False => ExpressionTree::Primary(Primary::False),
            Token::Number(n, _) => ExpressionTree::Primary(Primary::Number(n)),
            Token::String(s) => ExpressionTree::Primary(Primary::String(s)),
            Token::LeftParen => {
                let expr_tree =
                    ExpressionTree::Primary(Primary::Group(Box::new(parse_expr(tokens, 0)?)));
                if !tokens
                    .next()
                    .is_some_and(|token| token == Token::RightParen)
                {
                    return Err(ParseExpressionError::MissingRightParen);
                }
                expr_tree
            }
            Token::Identifier(ident) => {
                if tokens.next_if_eq(&Token::Equal).is_some() {
                    ExpressionTree::Assignment(ident, Box::new(parse_expr(tokens, 1)?))
                } else {
                    ExpressionTree::Primary(Primary::Identifier(ident))
                }
            }

            // prefix operator (Unary)
            Token::Minus => ExpressionTree::Unary(Unary::Minus(Box::new(parse_expr(tokens, 5)?))),
            Token::Bang => ExpressionTree::Unary(Unary::Bang(Box::new(parse_expr(tokens, 5)?))),
            token => return Err(ParseExpressionError::InvalidToken(token)),
        }
    } else {
        ExpressionTree::Primary(Primary::Nil)
    };

    // We parse the tokens until we hit something with a lower precedence.
    while let Some(next_token) = tokens.peek() {
        match next_token {
            Token::Star => {
                let bp = 5;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                // Here we want to pass the next items until we encounter something that have the same level of
                // precedence that the Star. If it's lower, for instance a +, we stop
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Factor(Factor::Star(Box::new(lhs), Box::new(rhs)));
            }
            Token::Slash => {
                let bp = 5;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Factor(Factor::Slash(Box::new(lhs), Box::new(rhs)));
            }
            Token::Plus => {
                let bp = 4;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Term(Term::Plus(Box::new(lhs), Box::new(rhs)));
            }
            Token::Minus => {
                let bp = 4;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Term(Term::Minus(Box::new(lhs), Box::new(rhs)));
            }
            Token::Less => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Comparison(Comparison::Less(Box::new(lhs), Box::new(rhs)));
            }
            Token::LessEqual => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs =
                    ExpressionTree::Comparison(Comparison::LessEqual(Box::new(lhs), Box::new(rhs)));
            }
            Token::Greater => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }

                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Comparison(Comparison::Greater(Box::new(lhs), Box::new(rhs)));
            }
            Token::GreaterEqual => {
                let bp = 3;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Comparison(Comparison::GreaterEqual(
                    Box::new(lhs),
                    Box::new(rhs),
                ));
            }

            Token::EqualEqual => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Equality(Equality::EqualEqual(Box::new(lhs), Box::new(rhs)));
            }
            Token::BangEqual => {
                let bp = 2;
                if bp > min_bp {
                    tokens.next();
                } else {
                    break;
                }
                let rhs = parse_expr(tokens, bp)?;
                lhs = ExpressionTree::Equality(Equality::BangEqual(Box::new(lhs), Box::new(rhs)));
            }
            _ => {
                break;
            }
        }
    }

    Ok(lhs)
}
// We only have left associativity (exept for prefix operator) so we can use only one binding power number

#[derive(Debug, PartialEq)]
pub enum ExpressionTree<'de> {
    Primary(Primary<'de>),
    Unary(Unary<'de>),
    Factor(Factor<'de>),
    Term(Term<'de>),
    Comparison(Comparison<'de>),
    Equality(Equality<'de>),
    Assignment(&'de str, Box<ExpressionTree<'de>>),
}

impl fmt::Display for ExpressionTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionTree::Primary(prim) => write!(f, "{prim}"),
            ExpressionTree::Unary(unary) => write!(f, "{unary}"),
            ExpressionTree::Factor(factor) => write!(f, "{factor}"),
            ExpressionTree::Term(term) => write!(f, "{term}"),
            ExpressionTree::Comparison(comparison) => write!(f, "{comparison}"),
            ExpressionTree::Equality(equality) => write!(f, "{equality}"),
            ExpressionTree::Assignment(ident, expr) => write!(f, "{ident} = {expr}"),
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
    Group(Box<ExpressionTree<'de>>),
    // A variable name
    Identifier(&'de str),
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
            Primary::Identifier(_) => todo!(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Unary<'de> {
    Bang(Box<ExpressionTree<'de>>),
    Minus(Box<ExpressionTree<'de>>),
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
    Slash(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    Star(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
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
    Minus(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    Plus(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
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
    Less(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    LessEqual(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    Greater(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    GreaterEqual(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
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
    EqualEqual(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
    BangEqual(Box<ExpressionTree<'de>>, Box<ExpressionTree<'de>>),
}

impl fmt::Display for Equality<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Equality::EqualEqual(left, right) => write!(f, "(== {left} {right})"),
            Equality::BangEqual(left, right) => write!(f, "(!= {left} {right})"),
        }
    }
}

#[derive(Debug)]
pub enum ParseExpressionError<'de> {
    InvalidToken(Token<'de>),
    MissingRightParen,
    MissingRightBrace,
}

impl<'de> std::error::Error for ParseExpressionError<'de> {}

impl fmt::Display for ParseExpressionError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseExpressionError::InvalidToken(token) => write!(f, "invalid token: {token}"),
            ParseExpressionError::MissingRightParen => write!(f, "missing right paren"),
            ParseExpressionError::MissingRightBrace => write!(f, "missing right brace"),
        }
    }
}
