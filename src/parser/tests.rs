#![cfg(test)]

use crate::ast::statement::Statement;
use crate::ast::Node;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_let_statements() {
    let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    assert_eq!(program.statements.len(), 3);

    let tests = vec![("x", 5), ("y", 10), ("foobar", 838383)];

    for (i, tt) in tests.iter().enumerate() {
        let stmt = &program.statements[i];
        test_let_statement(stmt, tt.0);
    }
}

fn test_let_statement(stmt: &Statement, name: &str) {
    assert_eq!(stmt.token_literal(), "let");

    if let Statement::Let(let_stmt) = stmt {
        assert_eq!(let_stmt.name.value, name);
        assert_eq!(let_stmt.name.token_literal(), name);
    } else {
        panic!(
            "Statement is not a Let statement, got {}",
            stmt.token_literal()
        );
    }
}
