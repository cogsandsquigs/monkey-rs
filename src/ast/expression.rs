use super::Node;

/// The `Expression` trait is implemented by all AST expression nodes. Note that the `expression_node`
/// method is not implemented here, as it is only used as guidance for the Go compiler in the original
/// implementation. Given that in Rust, we explicitly implement traits for types, this is not needed.
pub trait Expression: Node {}
