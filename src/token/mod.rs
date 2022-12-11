use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Token<'a> {
    /// The token type that this token is.
    r#type: TokenType,

    /// The literal value of the token. Using `&str` instead of `String` to avoid
    /// unnecessary allocations, as well to make significant that this is simply
    /// a slice of the input, and not a mutable copy of it. It doesn't need to be
    /// mutable, anyway.
    literal: &'a str,
}

#[allow(dead_code)] // The token types here are not used yet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::Eof => write!(f, "EOF"),
            TokenType::Ident => write!(f, "IDENT"),
            TokenType::Int => write!(f, "INT"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "FUNCTION"),
            TokenType::Let => write!(f, "LET"),
        }
    }
}
