use core::fmt::Debug;

/// The `Node` trait is implemented by all AST nodes. It provides a `token_literal` method, which
/// returns the literal value of the token that the node represents. This is used for debugging.
trait Node: Debug {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String;
}

/// The `Statement` trait is implemented by all AST statement nodes. Note that the `statement_node`
/// method is not implemented here, as it is only used as guidance for the Go compiler in the original
/// implementation. Given that in Rust, we explicitly implement traits for types, this is not needed.
trait Statement: Node {}

/// The `Expression` trait is implemented by all AST expression nodes. Note that the `expression_node`
/// method is not implemented here, as it is only used as guidance for the Go compiler in the original
/// implementation. Given that in Rust, we explicitly implement traits for types, this is not needed.
trait Expression: Node {}

/// The `Program` struct represents the root node of the AST. It contains a list of statements.
#[derive(Debug)]
struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            statements: self.statements.iter().map(|s| s.).collect(),
        }
    }
}
