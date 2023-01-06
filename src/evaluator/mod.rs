mod expressions;
mod statements;
pub mod tests;

use self::statements::eval_statement;
use crate::{ast::Program, object::Object};

/// Evaluates a `Program`.
pub fn eval(program: Program) -> Option<Object> {
    let mut result = None;

    for statement in program.statements {
        result = eval_statement(statement);
    }

    result
}
