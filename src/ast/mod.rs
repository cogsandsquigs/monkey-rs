pub mod expression;
pub mod statement;

use self::statement::Statement;
use core::fmt::Debug;

/// The `Node` trait is implemented by all AST nodes. It provides a `token_literal` method, which
/// returns the literal value of the token that the node represents. This is used for debugging.
pub trait Node: Debug {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String;
}

/// The `Program` struct represents the root node of the AST. It contains a list of statements.
#[derive(Debug, Clone)]
pub struct Program {
    /// A collection of statements that are contained in the program. Note that this is a `Vec` of
    /// `Rc` pointers to `Statement` trait objects, as 1) it allows us to clone `Program`, and 2) it
    /// allows us to store different types of statements in the same vector.
    statements: Vec<Statement>,
}

impl Node for Program {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }
}
