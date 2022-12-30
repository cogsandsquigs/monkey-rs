use std::fmt::Display;

use super::{statements::Statement, Node};
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

    /// The `Boolean` struct represents a boolean literal in the Monkey language. Note that in the
    /// original implementation, this is called `BooleanLiteral`.
    Boolean(Boolean),

    /// The `Prefix` struct represents a prefix expression in the Monkey language.
    Prefix(PrefixExpression),

    /// The `Infix` struct represents an infix expression in the Monkey language.
    Infix(InfixExpression),

    /// The `If` struct represents an `if` expression in the Monkey language.
    If(IfExpression),
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Self::Identifier(identifier) => identifier.token_literal(),
            Self::Integer(integer) => integer.token_literal(),
            Self::Boolean(boolean) => boolean.token_literal(),
            Self::Prefix(prefix) => prefix.token_literal(),
            Self::Infix(infix) => infix.token_literal(),
            Self::If(if_expression) => if_expression.token_literal(),
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

/// The `Boolean` struct represents a boolean literal in the Monkey language. Note that in the
/// original implementation, this is called `BooleanLiteral`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean {
    /// The `token` field is the token that the boolean literal represents.
    pub token: Token,

    /// The `value` field is the literal value of the boolean literal.
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `PrefixExpression` struct represents a prefix expression in the Monkey language. For example,
/// the expression `-5` is a prefix expression with the operator `-` and the right-hand side being the
/// integer literal `5`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpression {
    /// The `token` field is the token that the prefix expression represents.
    pub token: Token,

    /// The `operator` field is the operator of the prefix expression.
    pub operator: String,

    /// The `right` field is the right-hand side of the prefix expression.
    pub right: Box<Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `InfixExpression` struct represents an infix expression in the Monkey language. For example,
/// the expression `5 + 5` is an infix expression with the operator `+` and the left-hand side being
/// the integer literal `5` and the right-hand side being the integer literal `5`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfixExpression {
    /// The `token` field is the token that the infix expression represents.
    pub token: Token,

    /// The `left` field is the left-hand side of the infix expression.
    pub left: Box<Expression>,

    /// The `operator` field is the operator of the infix expression.
    pub operator: String,

    /// The `right` field is the right-hand side of the infix expression.
    pub right: Box<Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `IfExpression` struct represents an `if` expression in the Monkey language. For example, the
/// expression `if (x < y) { x } else { y }` is an `if` expression with the condition being the
/// infix expression `x < y`, the consequence being the block statement `{ x }`, and the alternative
/// being the block statement `{ y }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpression {
    /// The `token` field is the token that the `if` expression represents.
    pub token: Token,

    /// The `condition` field is the condition of the `if` expression.
    pub condition: Box<Expression>,

    /// The `consequence` field is the consequence of the `if` expression.
    pub consequence: BlockStatement,

    /// The `alternative` field is the alternative of the `if` expression.
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `BlockStatement` struct represents a block statement in the Monkey language. For example, the
/// block statement `{ x }` is a block statement with the `statements` field containing the
/// expression statement `x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStatement {
    /// The `token` field is the token that the block statement represents.
    pub token: Token,

    /// The `statements` field is the statements of the block statement.
    pub statements: Vec<Statement>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Prefix(prefix) => write!(f, "{}", prefix),
            Self::Infix(infix) => write!(f, "{}", infix),
            Self::If(if_expression) => write!(f, "{}", if_expression),
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

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {}", self.condition, self.consequence)?;
        if let Some(alternative) = &self.alternative {
            write!(f, " else {}", alternative)?;
        }
        Ok(())
    }
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
