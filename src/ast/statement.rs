// TODO: Remove `token` fields from structs, as they can be made redundant because we know what token made
// the statement.

use super::{expression::Expression, Node};
use crate::token::Token;
use std::rc::Rc;

/// The `Statement` trait is implemented by all AST statement nodes. Note that the `statement_node`
/// method is not implemented here, as it is only used as guidance for the Go compiler in the original
/// implementation. Given that in Rust, we explicitly implement traits for types, this is not needed.
pub trait Statement: Node {}

/// Denotes a `let` statement. It contains the name of the variable that is being assigned to, and the
/// expression that is being assigned to it.
#[derive(Debug, Clone)]
pub struct LetStatement {
    /// The token that represents the `let` keyword.
    token: Token,
    /// The name of the variable that is being assigned to.
    name: Identifier,
    /// The expression that is being assigned to the variable.
    value: Rc<dyn Expression>,
}

impl Node for LetStatement {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// The token that represents the identifier.
    token: Token,
    /// The value of the identifier.
    value: String,
}

impl Node for Identifier {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {}
