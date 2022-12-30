#![cfg(test)]

use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;

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
        test_let_statement(stmt, name);
    }
}

/// Helper function to test a `LetStatement` statement.
fn test_let_statement(stmt: &Statement, name: &str) {
    if let Statement::LetStatement(let_stmt) = stmt {
        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token_literal(), name);
    } else {
        panic!(
            "Statement is not a LetStatement statement, got {}",
            stmt.token_literal()
        );
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

    let Expression::Identifier(ident) = &stmt.expression else {
        panic!(
            "Expression is not an Identifier expression, got {}",
            stmt.expression.token_literal()
        );
    };

    assert_eq!(ident.value, "foobar");

    assert_eq!(ident.token_literal(), "foobar");
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

    let Expression::Integer(int) = &stmt.expression else {
        panic!(
            "Expression is not an IntegerLiteral expression, got {}",
            stmt.expression.token_literal()
        );
    };

    assert_eq!(int.value, 5);
}

/// Tests the parsing of prefix expressions.
#[test]
fn test_prefix_expressions() {
    let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

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

        test_integer(&prefix.right, value);

        assert_eq!(prefix.token_literal(), operator);
    }
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

/// Tests the parsing of infix expressions. Note that in the original implementation, the test
/// is called `TestParsingInfixExpressions`, but I changed the general name to fit with the ast
/// struct name.
#[test]
fn test_infix_expressions() {
    let infix_tests = vec![
        ("5 + 5;", 5, "+", 5),
        ("5 - 5;", 5, "-", 5),
        ("5 * 5;", 5, "*", 5),
        ("5 / 5;", 5, "/", 5),
        ("5 > 5;", 5, ">", 5),
        ("5 < 5;", 5, "<", 5),
        ("5 == 5;", 5, "==", 5),
        ("5 != 5;", 5, "!=", 5),
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

        let Expression::Infix(infix) = &stmt.expression else {
            panic!(
                "Expression is not an Infix expression, got {}",
                stmt.expression.token_literal()
            );
        };

        test_integer(&infix.left, left_value);

        assert_eq!(infix.operator, operator);

        test_integer(&infix.right, right_value);

        assert_eq!(infix.token_literal(), operator);
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
    ];

    for (input, expected) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.to_string(), expected);
    }
}
