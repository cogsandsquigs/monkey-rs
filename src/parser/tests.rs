#![cfg(test)]

use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::any::Any;

/// Helper function to check for any errors in the parser.
fn check_parser_errors(parser: &Parser) {
    let errors = parser.errors();

    if errors.is_empty() {
        return;
    }

    println!("parser has {} errors", errors.len());

    for error in errors {
        println!("parser error: {}", error);
    }

    panic!("parser has errors");
}

/// Helper function to test an `Integer` expression.
fn test_integer(expr: &Expression, value: i64) {
    if let Expression::Integer(int) = expr {
        assert_eq!(int.value, value);
        assert_eq!(int.token_literal(), value.to_string());
    } else {
        panic!(
            "Expression is not an IntegerLiteral expression, got {}",
            expr.token_literal()
        );
    }
}

/// Helper function to test a `Boolean` expression.
fn test_boolean(expr: &Expression, value: bool) {
    if let Expression::Boolean(b) = expr {
        assert_eq!(b.value, value);
        assert_eq!(b.token_literal(), value.to_string());
    } else {
        panic!(
            "Expression is not an IntegerLiteral expression, got {}",
            expr.token_literal()
        );
    }
}

/// Helper function to test an `Identifier` expression.
fn test_identifier(expr: &Expression, value: &str) {
    if let Expression::Identifier(ident) = expr {
        assert_eq!(ident.value, value);
        assert_eq!(ident.token_literal(), value);
    } else {
        panic!(
            "Expression is not an Identifier expression, got {}",
            expr.token_literal()
        );
    }
}

/// Helper function to test literal expressions.
fn test_literal(expr: &Expression, expected: &dyn Any) {
    if expected.is::<i64>() {
        test_integer(expr, *expected.downcast_ref::<i64>().unwrap());
    } else if expected.is::<i32>() {
        test_integer(expr, *expected.downcast_ref::<i32>().unwrap() as i64);
    } else if expected.is::<bool>() {
        test_boolean(expr, *expected.downcast_ref::<bool>().unwrap());
    } else if expected.is::<&str>() {
        test_identifier(expr, expected.downcast_ref::<&str>().unwrap());
    } else {
        panic!("Type of expression {} not handled", expr);
    }
}

/// Helper function to test infix expressions.
fn test_infix(expr: &Expression, left: &dyn Any, operator: &str, right: &dyn Any) {
    let Expression::Infix(infix) = expr else {
        panic!(
            "Expression is not an Infix expression, got {}",
            expr.token_literal()
        );
    };

    test_literal(infix.left.as_ref(), left);
    assert_eq!(infix.operator, operator);
    test_literal(infix.right.as_ref(), right);
}

#[test]
fn test_let_statements() {
    let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "Expected 3 statements, but got {} statements",
        program.statements.len()
    );

    let tests = vec![("x", 5), ("y", 10), ("foobar", 838383)];

    for (i, (name, value)) in tests.iter().enumerate() {
        let stmt = &program.statements[i];

        let Statement::LetStatement(let_stmt) = stmt else {
            panic!(
                "Statement is not a LetStatement statement, got {}",
                stmt.token_literal()
            );
        };

        test_identifier(&Expression::Identifier(let_stmt.name.clone()), name);

        test_literal(let_stmt.value.as_ref().unwrap(), value);
    }
}

#[test]
fn test_return_statements() {
    let input = r#"
return 5;
return 10;
return 993322;
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        3,
        "Expected 3 statements, but got {} statements",
        program.statements.len()
    );

    for stmt in program.statements {
        let Statement::ReturnStatement(return_stmt) = stmt else {
            panic!(
                "Statement is not a ReturnStatement statement, got {}",
                stmt.token_literal()
            );
        };

        assert_eq!(return_stmt.token_literal(), "return");
    }
}

/// Tests that identifiers are parsed correctly by the expression parser.
#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::ExpressionStatement(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    test_identifier(&stmt.expression, "foobar");
}

/// Tests the parsing of integer literals. Note that in the original implementation, the test
/// is called `TestIntegerLiteralExpression`, but I changed the general name to fit with the
/// ast struct name.
#[test]
fn test_integer_expression() {
    let input = "5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::ExpressionStatement(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    test_integer(&stmt.expression, 5);
}

/// Tests the parsing of boolean literals.
#[test]
fn test_boolean_expression() {
    let input = r"
true;
false;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();
    check_parser_errors(&parser);

    assert_eq!(
        program.statements.len(),
        2,
        "Expected 2 statements, but got {} statements",
        program.statements.len()
    );

    for (i, expected) in vec![true, false].iter().enumerate() {
        let Statement::ExpressionStatement(stmt) = &program.statements[i] else {
            panic!(
                "Statement is not an ExpressionStatement statement, got {}",
                program.statements[i].token_literal()
            );
        };

        test_boolean(&stmt.expression, *expected);
    }
}

/// Tests the parsing of prefix expressions.
#[test]
fn test_prefix_expressions() {
    let prefix_tests: Vec<(&str, &str, &dyn Any)> = vec![
        ("-15;", "-", &15),
        ("!true;", "!", &true),
        ("!false;", "!", &false),
    ];

    for (input, operator, value) in prefix_tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::ExpressionStatement(stmt) = &program.statements[0] else {
            panic!(
                "Statement is not an ExpressionStatement statement, got {}",
                program.statements[0].token_literal()
            );
        };

        let Expression::Prefix(prefix) = &stmt.expression else {
            panic!(
                "Expression is not a Prefix expression, got {}",
                stmt.expression.token_literal()
            );
        };

        assert_eq!(prefix.operator, operator);

        test_literal(&prefix.right, value);

        assert_eq!(prefix.token_literal(), operator);
    }
}

/// Tests the parsing of infix expressions. Note that in the original implementation, the test
/// is called `TestParsingInfixExpressions`, but I changed the general name to fit with the ast
/// struct name.
#[test]
fn test_infix_expressions() {
    let infix_tests: Vec<(&str, &dyn Any, &str, &dyn Any)> = vec![
        ("5 + 5;", &5, "+", &5),
        ("5 - 5;", &5, "-", &5),
        ("5 * 5;", &5, "*", &5),
        ("5 / 5;", &5, "/", &5),
        ("5 > 5;", &5, ">", &5),
        ("5 < 5;", &5, "<", &5),
        ("5 == 5;", &5, "==", &5),
        ("5 != 5;", &5, "!=", &5),
        ("true == true", &true, "==", &true),
        ("true != false", &true, "!=", &false),
        ("false == false", &false, "==", &false),
    ];

    for (input, left_value, operator, right_value) in infix_tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::ExpressionStatement(stmt) = &program.statements[0] else {
            panic!(
                "Statement is not an ExpressionStatement statement, got {}",
                program.statements[0].token_literal()
            );
        };

        test_infix(&stmt.expression, left_value, operator, right_value);
    }
}

// Tests the parsing of precedence of operators.
#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        // ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        // (
        //     "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
        //     "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        // ),
        // (
        //     "add(a + b + c * d / f + g)",
        //     "add((((a + b) + ((c * d) / f)) + g))",
        // ),
        // (
        //     "a * [1, 2, 3, 4][b * c] * d",
        //     "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        // ),
        // (
        //     "add(a * b[2], b[1], 2 * [1, 2][1])",
        //     "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        // ),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.to_string(), expected);
    }
}
