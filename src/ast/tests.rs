#![cfg(test)]

use crate::token::{Token, TokenType};

use super::{
    expressions::{Expression, Identifier},
    statements::{LetStatement, Statement},
    Program,
};

#[test]
fn test_formatting() {
    let ast = Program {
        statements: vec![Statement::LetStatement(LetStatement {
            token: Token {
                r#type: TokenType::Let,
                literal: "let".to_string(),
            },
            name: Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: "myVar".to_string(),
                },
                value: "myVar".to_string(),
            },
            value: Some(Box::new(Expression::Identifier(Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: "anotherVar".to_string(),
                },
                value: "anotherVar".to_string(),
            }))),
        })],
    };

    assert_eq!(ast.to_string(), "let myVar = anotherVar;".to_string());
}
