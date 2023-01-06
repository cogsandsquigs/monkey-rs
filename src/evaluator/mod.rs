pub mod tests;

use crate::{
    ast::{expressions::Expression, statements::Statement, Program},
    object::{boolean::Boolean, integer::Integer, Object},
};

/// Evaluates a `Program`.
pub fn eval(program: Program) -> Option<Object> {
    let mut result = None;

    for statement in program.statements {
        result = eval_statement(statement);
    }

    result
}

/// Evaluates a single `Statement`.
fn eval_statement(statement: Statement) -> Option<Object> {
    match statement {
        Statement::Expression(expression) => eval_expression(expression.expression),

        _ => todo!(),
    }
}

/// Evaluates an `Expression`.
fn eval_expression(expression: Expression) -> Option<Object> {
    match expression {
        Expression::Integer(integer) => Some(Object::Integer(Integer::new(integer.value))),

        Expression::Boolean(boolean) => Some(Object::Boolean(Boolean::new(boolean.value))),

        _ => todo!(),
    }
}
