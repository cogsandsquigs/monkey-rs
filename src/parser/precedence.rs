use std::collections::HashMap;

use crate::token::TokenType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,   // ==
    Ordering, // > or <. Note that in the original implementation, this is called `LessGreater`.
    Sum,      // +
    Product,  // *
    Prefix,   // -X or !X
    Call,     // myFunction(X)
}

pub fn precedence_of(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Eq => Precedence::Equals,
        TokenType::NotEq => Precedence::Equals,
        TokenType::Lt => Precedence::Ordering,
        TokenType::Gt => Precedence::Ordering,
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Slash => Precedence::Product,
        TokenType::Star => Precedence::Product,
        _ => Precedence::Lowest,
    }
}
