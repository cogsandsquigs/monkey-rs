#![cfg(test)]

use crate::ast::expressions::Expression;
use crate::ast::statements::Statement;
use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::any::Any;

/// Helper function to test an `IntegerLiteral` expression.
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

/// Helper function to test a `BooleanLiteral` expression.
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

    assert_eq!(infix.operator.token_literal(), operator);

    assert_eq!(infix.operator.to_string(), operator);

    test_literal(infix.right.as_ref(), right);
}

#[test]
fn test_let_statements() {
    let tests: Vec<(&str, &str, &dyn Any)> = vec![
        ("let x = 5;", "x", &5),
        ("let y = true;", "y", &true),
        ("let foobar = y;", "foobar", &"y"),
    ];

    for (input, expected_identifier, expected_value) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::Let(stmt) = &program.statements[0] else {
            panic!(
                "Statement is not a LetStatement statement, got {}",
                program.statements[0].token_literal()
            );
        };

        assert_eq!(stmt.token_literal(), "let");
        assert_eq!(stmt.name.value, expected_identifier);
        assert_eq!(stmt.name.token_literal(), expected_identifier);

        test_literal(&stmt.value, expected_value);
    }
}

#[test]
fn test_return_statements() {
    let tests: Vec<(&str, &dyn Any)> = vec![
        ("return 5;", &5),
        ("return true;", &true),
        ("return foobar;", &"foobar"),
    ];

    for (input, expected_value) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::Return(stmt) = &program.statements[0] else {
            panic!(
                "Statement is not a ReturnStatement statement, got {}",
                program.statements[0].token_literal()
            );
        };

        assert_eq!(stmt.token_literal(), "return");

        test_literal(&stmt.return_value, expected_value);
    }
}

/// Tests that identifiers are parsed correctly by the expression parser.
#[test]
fn test_identifier_expression() {
    let input = "foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
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

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
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

    assert_eq!(
        program.statements.len(),
        2,
        "Expected 2 statements, but got {} statements",
        program.statements.len()
    );

    for (i, expected) in vec![true, false].iter().enumerate() {
        let Statement::Expression(stmt) = &program.statements[i] else {
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

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::Expression(stmt) = &program.statements[0] else {
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

        assert_eq!(prefix.operator.token_literal(), operator);

        assert_eq!(prefix.operator.r#type.to_string(), operator);

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

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::Expression(stmt) = &program.statements[0] else {
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
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        // (
        //     "a * [1, 2, 3, 4][b * c] * d",
        //     "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        // ),
        // (
        //     "add(a * b[2], b[1], 2 * [1, 2][1])",
        //     "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        // ),
    ];

    for (i, (input, expected)) in tests.into_iter().enumerate() {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(program.to_string(), expected, "Test {} failed", i);
    }
}

/// Testing the parsing of if expressions.
#[test]
fn test_if_expressions() {
    let input = "if (x < y) { x }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    let Expression::If(if_expr) = &stmt.expression else {
        panic!(
            "Expression is not an IfExpression expression, got {}",
            stmt.expression.token_literal()
        );
    };

    test_infix(&if_expr.condition, &"x", "<", &"y");

    assert_eq!(
        if_expr.consequence.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        if_expr.consequence.statements.len()
    );

    let Statement::Expression(consequence) = &if_expr.consequence.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            if_expr.consequence.statements[0].token_literal()
        );
    };

    test_identifier(&consequence.expression, "x");

    assert!(if_expr.alternative.is_none());
}

/// Tests the parsing of if-else expressions.
#[test]
fn test_if_else_expressions() {
    let input = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    let Expression::If(if_expr) = &stmt.expression else {
        panic!(
            "Expression is not an IfExpression expression, got {}",
            stmt.expression.token_literal()
        );
    };

    test_infix(&if_expr.condition, &"x", "<", &"y");

    assert_eq!(
        if_expr.consequence.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        if_expr.consequence.statements.len()
    );

    let Statement::Expression(consequence) = &if_expr.consequence.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            if_expr.consequence.statements[0].token_literal()
        );
    };

    test_identifier(&consequence.expression, "x");

    assert!(if_expr.alternative.is_some());

    let Statement::Expression(alternative) = if_expr.alternative.as_ref().unwrap().statements[0].clone() else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            if_expr.alternative.as_ref().unwrap().statements[0].token_literal()
        );
    };

    test_identifier(&alternative.expression, "y");
}

/// Test the parsing of function literals.
#[test]
fn test_function_literals() {
    let input = "fn(x, y) { x + y; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    let Expression::Function(function) = &stmt.expression else {
        panic!(
            "Expression is not a FunctionLiteral expression, got {}",
            stmt.expression.token_literal()
        );
    };

    assert_eq!(function.parameters.len(), 2);

    assert_eq!(function.parameters[0].token_literal(), "x");

    assert_eq!(function.parameters[1].token_literal(), "y");

    assert_eq!(
        function.body.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        function.body.statements.len()
    );

    let Statement::Expression(body) = &function.body.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            function.body.statements[0].token_literal()
        );
    };

    test_infix(&body.expression, &"x", "+", &"y");
}

/// Tests the parsing of function parameters.
#[test]
fn test_function_parameter_parsing() {
    let tests = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(
            program.statements.len(),
            1,
            "Expected 1 statement, but got {} statements",
            program.statements.len()
        );

        let Statement::Expression(stmt) = &program.statements[0] else {
            panic!(
                "Statement is not an ExpressionStatement statement, got {}",
                program.statements[0].token_literal()
            );
        };

        let Expression::Function(function) = &stmt.expression else {
            panic!(
                "Expression is not a FunctionLiteral expression, got {}",
                stmt.expression.token_literal()
            );
        };

        assert_eq!(function.parameters.len(), expected.len());

        for (i, ident) in expected.iter().enumerate() {
            assert_eq!(function.parameters[i].token_literal(), *ident);
        }
    }
}

/// Tests the parsing of call expressions.
#[test]
fn test_call_expressions() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program().unwrap();

    assert_eq!(
        program.statements.len(),
        1,
        "Expected 1 statement, but got {} statements",
        program.statements.len()
    );

    let Statement::Expression(stmt) = &program.statements[0] else {
        panic!(
            "Statement is not an ExpressionStatement statement, got {}",
            program.statements[0].token_literal()
        );
    };

    let Expression::Call(call) = &stmt.expression else {
        panic!(
            "Expression is not a CallExpression expression, got {}",
            stmt.expression.token_literal()
        );
    };

    test_identifier(&call.function, "add");

    assert_eq!(call.arguments.len(), 3);

    test_integer(&call.arguments[0], 1);

    test_infix(&call.arguments[1], &2, "*", &3);

    test_infix(&call.arguments[2], &4, "+", &5);
}
