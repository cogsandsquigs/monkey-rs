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

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let_statement(stmt, tt.0);
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
