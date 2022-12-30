use std::fmt::Display;

use super::Node;
use crate::token::Token;

/// An expression is a piece of code that evaluates to a value. For example, `5 + 5` is an expression
/// that evaluates to the value `10`. Note that while the original implementation uses raw `struct`s,
/// I have chosen to use `enum`s, as it allows us to store different types of expressions in the same
/// vector without having to use `Box` or `Rc` pointers and jumping through hoops/not allowing us to
/// clone `Program`. Also, if we were to directly encode the `Identifier` struct into the `Expression`
/// enum, we would have to use `Box` or `Rc` pointers around an `Expression` enum, which would be a
/// pain to work with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// The `Identifier` struct represents an identifier in the Monkey language.
    Identifier(Identifier),

    /// The `Integer` struct represents an integer literal in the Monkey language. Note that in the
    /// original implementation, this is called `IntegerLiteral`.
    Integer(Integer),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.token_literal(),
            Self::Integer(integer) => integer.token_literal(),
        }
    }
}

/// The `Identifier` struct represents an identifier in the Monkey language. It contains a `token`
/// field, which is the token that the identifier represents, and a `value` field, which is the
/// literal value of the identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    /// The `token` field is the token that the identifier represents.
    pub token: Token,

    /// The `value` field is the literal value of the identifier.
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `Integer` struct represents an integer literal in the Monkey language. Note that in the
/// original implementation, this is called `IntegerLiteral`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    /// The `token` field is the token that the integer literal represents.
    pub token: Token,

    /// The `value` field is the literal value of the integer literal.
    pub value: i64,
}

impl Node for Integer {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
