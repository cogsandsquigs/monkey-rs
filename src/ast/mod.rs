pub mod expressions;
pub mod statements;
mod tests;

use self::statements::Statement;
use core::fmt::Debug;
use std::fmt::Display;

/// The `Node` trait is implemented by all AST nodes. It provides a `token_literal` method, which
/// returns the literal value of the token that the node represents. This is used for debugging.
pub trait Node: Debug + Display {
    /// Returns the literal value of the token that the node represents. This is used for debugging.
    fn token_literal(&self) -> String;
}

/// The `Program` struct represents the root node of the AST. It contains a list of statements.
#[derive(Debug, Clone)]
pub struct Program {
    /// A collection of statements that are contained in the program.
    pub statements: Vec<Statement>,
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

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }

        Ok(())
    }
}
