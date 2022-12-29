use core::fmt::Display;

/// The token type that is used in the lexer. This contains both the type of the
/// token (as `TokenType`), as well as the string literal value that the token was
/// created from.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Token {
    /// The token type that this token is.
    pub r#type: TokenType,

    /// The literal value of the token.
    pub literal: String,
}

/// Public API for the `Token` struct.
impl Token {
    /// Creates a new `Token` from the given `TokenType` and literal value.
    pub fn new<S: ToString>(r#type: TokenType, literal: S) -> Self {
        Self {
            r#type,

            // Why do this? Because 1) we don't want to allocate a new `String` for
            // every token, and 2) it makes it easier when defining a token, as defining
            // a `&str` literal is easier than defining a `String` literal.
            literal: literal.to_string(),
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
#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    #[default]
    EOF,
    Illegal,

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

    Eq,
    NotEq,

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
        write!(
            f,
            "{}",
            match self {
                TokenType::Illegal => "ILLEGAL",
                TokenType::EOF => "EOF",
                TokenType::Ident => "IDENT",
                TokenType::Int => "INT",
                TokenType::Assign => "=",
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                TokenType::Bang => "!",
                TokenType::Star => "*",
                TokenType::Slash => "/",
                TokenType::Lt => "<",
                TokenType::Gt => ">",
                TokenType::Eq => "==",
                TokenType::NotEq => "!=",
                TokenType::Comma => ",",
                TokenType::Semicolon => ";",
                TokenType::LParen => "(",
                TokenType::RParen => ")",
                TokenType::LBrace => "{",
                TokenType::RBrace => "}",
                TokenType::Function => "FUNCTION",
                TokenType::Let => "LET",
                TokenType::If => "IF",
                TokenType::Else => "ELSE",
                TokenType::Return => "RETURN",
                TokenType::True => "TRUE",
                TokenType::False => "FALSE",
            }
        )
    }
}
