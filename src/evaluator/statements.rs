use super::expressions::eval_expression;
use crate::{ast::statements::Statement, object::Object};

/// Evaluates a single `Statement`.
pub fn eval_statement(statement: Statement) -> Option<Object> {
    match statement {
        Statement::Expression(expression) => eval_expression(expression.expression),

        _ => todo!(),
    }
}
