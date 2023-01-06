#![cfg(test)]

use crate::{
    ast::Program,
    lexer::Lexer,
    object::{Object, Objective},
    parser::Parser,
};

/// Parses the input source code
fn parse(input: &str) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program().unwrap()
}

/// Tests an integer object
fn test_integer_object(obj: Object, value: i64) {
    match obj {
        Object::Integer(integer) => assert_eq!(integer.value, value),
        _ => panic!("Object is not an Integer, found {:?}", obj.object_type()),
    }
}

/// Tests the evaluation of an integer expression
#[test]
fn test_eval_integer_expression() {
    let tests = vec![("5", 5), ("10", 10)];

    for (input, expected) in tests {
        let program = parse(input);
        let obj = eval(program);

        test_integer_object(obj, expected);
    }
}
