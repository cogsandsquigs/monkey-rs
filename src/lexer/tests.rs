#![cfg(test)]

use super::Lexer;
use crate::token::{Token, TokenType};

#[test]
fn next_token() {
    let input = "=+(){},;";

    let tests = vec![
        Token {
            r#type: TokenType::Assign,
            literal: "=",
        },
        Token {
            r#type: TokenType::Plus,
            literal: "+",
        },
        Token {
            r#type: TokenType::LParen,
            literal: "(",
        },
        Token {
            r#type: TokenType::RParen,
            literal: ")",
        },
        Token {
            r#type: TokenType::LBrace,
            literal: "{",
        },
        Token {
            r#type: TokenType::RBrace,
            literal: "}",
        },
        Token {
            r#type: TokenType::Comma,
            literal: ",",
        },
        Token {
            r#type: TokenType::Semicolon,
            literal: ";",
        },
        Token {
            r#type: TokenType::Eof,
            literal: "",
        },
    ];

    let mut lexer = Lexer::new(input);

    for (i, tt) in tests.iter().enumerate() {
        let token = lexer.next_token();

        assert_eq!(token.r#type, tt.r#type, "tests[{}] failed - type wrong.", i);

        assert_eq!(token, *tt, "tests[{}] failed - literal wrong.", i);
    }
}
