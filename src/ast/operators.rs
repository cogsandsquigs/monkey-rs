use super::Node;
use crate::token::Token;
use std::fmt::Display;

/// a `PrefixOperator` is a token that can be used in an expression. `Operator`s are used to build
/// `Expression`s, and operate (pun intended) on values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrefixOperator {
    /// The `Token` that the `Operator` was created from.
    pub token: Token,

    /// The operator itself.
    pub r#type: PrefixOperatorType,
}

impl PrefixOperator {
    /// Creates a new `Operator` from a `Token`.
    pub fn new(token: Token, r#type: PrefixOperatorType) -> Self {
        Self { token, r#type }
    }
}

impl Node for PrefixOperator {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

/// Prefix/Unary operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrefixOperatorType {
    /// `!`
    Bang,
    /// `-` (unary)
    Neg,
}

impl Display for PrefixOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bang => write!(f, "!"),
            Self::Neg => write!(f, "-"),
        }
    }
}

/// a `InfixOperator` is a token that can be used in an expression. `Operator`s are used to build
/// `Expression`s, and operate (pun intended) on values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfixOperator {
    /// The `Token` that the `Operator` was created from.
    pub token: Token,

    /// The operator itself.
    pub r#type: InfixOperatorType,
}

impl InfixOperator {
    /// Creates a new `Operator` from a `Token`.
    pub fn new(token: Token, r#type: InfixOperatorType) -> Self {
        Self { token, r#type }
    }
}

impl Node for InfixOperator {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

/// Infix/Binary operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfixOperatorType {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `==`
    Eq,
    /// `!=`
    NotEq,
    /// `<`
    Lt,
    /// `>`
    Gt,
}

impl Display for InfixOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Eq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
        }
    }
}
