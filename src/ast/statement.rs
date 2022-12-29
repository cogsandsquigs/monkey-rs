use std::fmt::Display;

use super::{
    expression::{Expression, Identifier},
    Node,
};
use crate::token::Token;

/// The `Statement` enum represents a statement in the Monkey language. Note that while in the original
/// implementation, the `Statement` trait was implemented by the `LetStatement` struct, I have chosen to
/// implement the `Statement` trait via the `Statement` enum, as it allows us to store different types of
/// statements in the same vector without having to use `Box` or `Rc` pointers and jumping through hoops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// The `LetStatement` struct represents a `let` statement in the Monkey language.
    LetStatement(LetStatement),

    /// The `ReturnStatement` struct represents a `return` statement in the Monkey language.
    ReturnStatement(ReturnStatement),

    /// The `ExpressionStatement` struct represents an expression statement in the Monkey language.
    ExpressionStatement(ExpressionStatement),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::LetStatement(let_statement) => let_statement.token_literal(),
            Self::ReturnStatement(return_statement) => return_statement.token_literal(),
            Self::ExpressionStatement(expression_statement) => expression_statement.token_literal(),
        }
    }
}

/// The `LetStatement` struct represents a `let` statement in the Monkey language. It contains a
/// `token` field, which is the `let` token, a `name` field, which is the identifier that is being
/// assigned to, and a `value` field, which is the expression that is being assigned to the identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetStatement {
    /// The `token` field is the `let` token.
    pub token: Token,

    /// The `name` field is the identifier that is being assigned to.
    pub name: Identifier,

    /// The `value` field is the expression that is being assigned to the identifier.
    /// TODO: Get rid of the `Option` here, only necessary b/c we aren't parsing expressions yet.
    pub value: Option<Box<Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `ReturnStatement` struct represents a `return` statement in the Monkey language. It contains a
/// `token` field, which is the `return` token, and a `return_value` field, which is the expression
/// that is being returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement {
    /// The `token` field is the `return` token.
    pub token: Token,

    /// The `value` field is the expression that is being returned.
    /// TODO: Get rid of the `Option` here, only necessary b/c we aren't parsing expressions yet.
    pub value: Option<Box<Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `ExpressionStatement` struct represents an expression statement in the Monkey language. It contains a
/// `token` field, which is the first token of the expression, and a `expression` field, which is the expression
/// itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement {
    /// The `token` field is the first token of the expression.
    pub token: Token,

    /// The `expression` field is the expression itself.
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LetStatement(let_statement) => write!(f, "{}", let_statement),
            Self::ReturnStatement(return_statement) => write!(f, "{}", return_statement),
            Self::ExpressionStatement(expression_statement) => {
                write!(f, "{}", expression_statement)
            }
        }
    }
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} = ", self.token_literal(), self.name)?;

        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }

        write!(f, ";")
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.token_literal())?;

        if let Some(value) = &self.value {
            write!(f, "{}", value)?;
        }

        write!(f, ";")
    }
}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
