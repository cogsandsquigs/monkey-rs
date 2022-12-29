use super::{
    expression::{Expression, Identifier},
    Node,
};
use crate::token::Token;

/// The `Statement` enum represents a statement in the Monkey language. Note that while in the original
/// implementation, the `Statement` trait was implemented by the `Let` struct, I have chosen to
/// implement the `Statement` trait via the `Statement` enum, as it allows us to store different types of
/// statements in the same vector without having to use `Box` or `Rc` pointers and jumping through hoops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    /// The `Let` struct represents a `let` statement in the Monkey language. It contains a
    /// `token` field, which is the `let` token, a `name` field, which is the identifier that is being
    /// assigned to, and a `value` field, which is the expression that is being assigned to the identifier.
    Let(Let),
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(let_statement) => let_statement.token_literal(),
        }
    }
}

/// The `Let` struct represents a `let` statement in the Monkey language. It contains a
/// `token` field, which is the `let` token, a `name` field, which is the identifier that is being
/// assigned to, and a `value` field, which is the expression that is being assigned to the identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    /// The `token` field is the `let` token.
    pub token: Token,

    /// The `name` field is the identifier that is being assigned to.
    pub name: Identifier,

    /// The `value` field is the expression that is being assigned to the identifier.
    pub value: Option<Box<Expression>>,
}

impl Node for Let {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
