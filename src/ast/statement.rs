use super::Node;

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
    value: Expression,
}
