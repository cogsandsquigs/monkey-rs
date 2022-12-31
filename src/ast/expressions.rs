use super::{statements::BlockStatement, Node};
use crate::token::Token;
use std::fmt::Display;

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

    /// The `IntegerLiteral` struct represents an integer literal in the Monkey language.
    Integer(IntegerLiteral),

    /// The `BooleanLiteral` struct represents a boolean literal in the Monkey language.
    Boolean(BooleanLiteral),

    /// The `FunctionLiteral` struct represents a function literal in the Monkey language.
    Function(FunctionLiteral),

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
            Self::Function(function) => function.token_literal(),
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

/// The `IntegerLiteral` struct represents an integer literal in the Monkey language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteral {
    /// The `token` field is the token that the integer literal represents.
    pub token: Token,

    /// The `value` field is the literal value of the integer literal.
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `BooleanLiteral` struct represents a boolean literal in the Monkey language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanLiteral {
    /// The `token` field is the token that the boolean literal represents.
    pub token: Token,

    /// The `value` field is the literal value of the boolean literal.
    pub value: bool,
}

impl Node for BooleanLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

/// The `FunctionLiteral` struct represents a function literal in the Monkey language. For example,
/// the expression `fn(x, y) { x + y; }` is a function literal with the parameters `x` and `y` and
/// the body being the expression `x + y`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionLiteral {
    /// The `token` field is the token that the function literal represents.
    pub token: Token,

    /// The `parameters` field is the parameters of the function literal.
    pub parameters: Vec<Identifier>,

    /// The `body` field is the body of the function literal.
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Function(function) => write!(f, "{}", function),
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

impl Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_literal())?;
        write!(
            f,
            "({})",
            self.parameters
                .iter()
                .map(Identifier::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        write!(f, " ")?;
        write!(f, "{}", self.body)
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
