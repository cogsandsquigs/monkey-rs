#![cfg(test)]

use crate::{
    ast::{Nodes, Program},
    lexer::Lexer,
    object::Object,
    parser::Parser,
};

use super::eval;

/// Parses the input source code
fn parse(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program().unwrap()
}

/// Tests an integer object
fn test_integer_object(obj: Option<Object>, value: i64) {
    match obj {
        Some(Object::Integer(integer)) => assert_eq!(integer.value, value),
        _ => panic!("Object is not an Integer, found {:?}", obj),
    }
}

/// Tests a boolean object
fn test_boolean_object(obj: Option<Object>, value: bool) {
    match obj {
        Some(Object::Boolean(boolean)) => assert_eq!(boolean.value, value),
        _ => panic!("Object is not a Boolean, found {:?}", obj),
    }
}

/// Tests the evaluation of an integer expression
#[test]
fn test_eval_integer_expression() {
    let tests = vec![("5", 5), ("10", 10)];

    for (input, expected) in tests {
        let program = parse(input);
        let obj = eval(Nodes::Program(program));

        test_integer_object(obj, expected);
    }
}

/// Tests the evaluation of a boolean expression
#[test]
fn test_eval_boolean_expression() {
    let tests = vec![("true", true), ("false", false)];

    for (input, expected) in tests {
        let program = parse(input);
        let obj = eval(Nodes::Program(program));

        test_boolean_object(obj, expected);
    }
}
