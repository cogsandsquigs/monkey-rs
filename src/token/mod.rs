use core::fmt::Display;

/// The token type that is used in the lexer. This contains both the type of the
/// token (as `TokenType`), as well as the string literal value that the token was
/// created from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    /// The token type that this token is.
    pub r#type: TokenType,

    /// The literal value of the token. Using `&str` instead of `String` to avoid
    /// unnecessary allocations, as well to make significant that this is simply
    /// a slice of the input, and not a mutable copy of it. It doesn't need to be
    /// mutable, anyway.
    pub literal: &'a str,
}

/// Public API for the `Token` struct.
impl Token<'_> {
    /// Creates a new `Token` from the given `TokenType` and literal value.
    pub fn new<S: ToString>(r#type: TokenType, literal: S) -> Self {
        Self {
            r#type,

            // Why do this? Because 1) we don't want to allocate a new `String` for
            // every token, and 2) it makes it easier when defining a token, as defining
            // a `&str` literal is easier than defining a `String` literal.
            literal: Box::leak(literal.to_string().into_boxed_str()),
        }
    }

    /// Creates a new `Token` from a given identifier `ident`. This is used when
    /// lexing an identifier, as we don't know if it is a keyword or not until
    /// we have lexed the entire identifier. Defaults to `TokenType::Ident`.
    pub fn from_ident(ident: String) -> Self {
        Self::new(
            match ident.as_str() {
                "fn" => TokenType::Function,
                "let" => TokenType::Let,
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "return" => TokenType::Return,
                _ => TokenType::Ident,
            },
            ident,
        )
    }
}

/// The token type that is used in the lexer. These are markers for the type of
/// token that is being used. Note that they do not contain the actual value of
/// the token, only the type.
#[allow(dead_code)] // The token types here are not used yet
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    // In the original implementation, this is called `Asterisk`. However, I
    // thought that `Star` is a better name.
    Star,
    Slash,

    Lt,
    Gt,

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
    If,
    Else,
    Return,
    True,
    False,
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
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Star => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Lt => write!(f, "<"),
            TokenType::Gt => write!(f, ">"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "FUNCTION"),
            TokenType::Let => write!(f, "LET"),
            TokenType::If => write!(f, "IF"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
        }
    }
}
