#![cfg(tests)]

#[test]
fn next_token() {
    let input = "=+(){{}},;";

    let expected = vec![
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
            literal: "{{",
        },
        Token {
            r#type: TokenType::RBrace,
            literal: "}}",
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

    let lexer = Lexer::new(input);

    for (i, tt) in expected.iter().enumerate() {}
}
