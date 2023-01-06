pub mod tests;

use crate::{
    ast::{expressions::Expression, statements::Statement, Nodes},
    object::{boolean::Boolean, integer::Integer, Object},
};

pub fn eval(node: Nodes) -> Option<Object> {
    match node {
        // Statements
        Nodes::Program(program) => eval_statements(program.statements),

        Nodes::Statement(statement) => match statement {
            Statement::Expression(expression) => eval(Nodes::Expression(expression.expression)),

            _ => todo!(),
        },

        // Expressions
        Nodes::Expression(expression) => match expression {
            Expression::Integer(integer) => Some(Object::Integer(Integer::new(integer.value))),

            Expression::Boolean(boolean) => Some(Object::Boolean(Boolean::new(boolean.value))),

            _ => todo!(),
        },
    }
}

fn eval_statements(statements: Vec<Statement>) -> Option<Object> {
    let mut result = None;

    for statement in statements {
        result = eval(Nodes::Statement(statement))
    }

    result
}
