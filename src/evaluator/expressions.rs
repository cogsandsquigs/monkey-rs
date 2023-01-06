use crate::{
    ast::{
        expressions::{Expression, PrefixExpression},
        operators::{PrefixOperator, PrefixOperatorType},
    },
    object::{boolean::Boolean, integer::Integer, null::Null, Object},
};

/// Evaluates an `Expression`.
pub fn eval_expression(expression: Expression) -> Option<Object> {
    match expression {
        Expression::Integer(integer) => Some(Object::Integer(Integer::new(integer.value))),

        Expression::Boolean(boolean) => Some(Object::Boolean(Boolean::new(boolean.value))),

        Expression::Prefix(PrefixExpression {
            right,
            operator: PrefixOperator {
                r#type: operator, ..
            },
            ..
        }) => {
            let right = eval_expression(*right)?;
            eval_prefix(operator, right)
        }

        _ => todo!(),
    }
}

/// Evaluates a `PrefixExpression`.
fn eval_prefix(operator: PrefixOperatorType, right: Object) -> Option<Object> {
    match operator {
        PrefixOperatorType::Bang => eval_op_bang(right),

        PrefixOperatorType::Neg => eval_op_neg(right),
    }
}

/// Evaluates the `!` operator. This operator defaults to `false`, except for `null` and `false`, which
/// it converts to `true`.
fn eval_op_bang(right: Object) -> Option<Object> {
    match right {
        Object::Boolean(Boolean { value }) => Some(Object::Boolean(Boolean::new(!value))),

        Object::Null(Null) => Some(Object::Boolean(Boolean::new(true))),

        _ => Some(Object::Boolean(Boolean::new(false))),
    }
}

/// Evaluates the `-` operator. This operator converts the `right` object to an `Integer` and negates
/// its value.
fn eval_op_neg(right: Object) -> Option<Object> {
    match right {
        Object::Integer(Integer { value }) => Some(Object::Integer(Integer::new(-value))),

        _ => None,
    }
}
