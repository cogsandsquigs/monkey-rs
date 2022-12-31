pub mod errors;
mod expressions;
mod precedence;
mod statements;
mod tests;

use self::{
    errors::Error,
    expressions::{InfixParseFn, PrefixParseFn},
};
use crate::{
    ast::Program,
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::collections::HashMap;

/// The `ParseResult` type is a shorthand for a `Result` type that returns a `()` error type. This is
/// used to simplify the return type of the `parse_*` functions.
/// TODO: This should probably be replaced with a custom error type.
type ParseResult<T> = Result<T, Error>;

/// The parser for the Monkey programming language. It takes a `Lexer` and parses it into an AST.
pub struct Parser {
    /// The `lexer` field is the `Lexer` that the parser is parsing.
    lexer: Lexer,

    /// The `current_token` field is the current token that the parser is looking at.
    current_token: Token,

    /// The `peek_token` field is the next token that the parser is looking at.
    peek_token: Token,

    /// The `prefix_parse_fns` field is a map of token types to prefix parse functions. This is used
    /// to determine how to parse a given token for a prefix expression.
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,

    /// The `infix_parse_fns` field is a map of token types to infix parse functions. This is used to
    /// determine how to parse a given token for an infix expression.
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

/// Public API for the `Parser` struct.
impl Parser {
    /// Creates a new `Parser`, given a `Lexer`. This also "primes" the parser by calling `next_token`
    /// twice, so that `current_token` and `peek_token` are both set.
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,

            current_token: Token::default(),
            peek_token: Token::default(),

            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register tokens with their respective parse functions.
        parser.register_tokens();

        // Prime the parser by calling `next_token` twice.
        parser.next_token();
        parser.next_token();

        parser
    }

    /// Parses the input from the `Lexer` into an AST.
    pub fn parse_program(&mut self) -> Result<Program, Vec<Error>> {
        let mut program = Program { statements: vec![] };
        let mut errors = vec![];

        while self.current_token.r#type != TokenType::EOF {
            let stmt = self.parse_statement();

            if let Ok(stmt) = stmt {
                program.statements.push(stmt);
            } else if let Err(err) = stmt {
                errors.push(err.clone());
                self.synchronize();
            }

            self.next_token();
        }

        if errors.is_empty() {
            Ok(program)
        } else {
            Err(errors)
        }
    }
}

/// Private API for the `Parser` struct.
impl Parser {
    /// When parsing code, sometimes we run into parsing errors. This function, `synchronize`, helps to recover
    /// from parsing errors by consuming code input until we reach a token that we can continue parsing from (i.e.
    /// a semicolon). This does not guarantee that the new position creates valid code, but does help in preventing
    /// large amounts of errors from being reported due to small syntax errors.
    fn synchronize(&mut self) {
        // We use `peek_token` here as the parsing loop calls `next_token` before parsing another statement.
        let mut token = self.peek_token.clone();

        // As long as there is more input to parse, parse it
        while token.r#type != TokenType::EOF {
            // If we reach a token that can end a statement, we can continue parsing. However, we
            // must advance the parser by one token, as our parsing functions expect that the current
            // token is whatever token they expect, not a statement-ending token.
            if matches!(token.r#type, TokenType::Semicolon | TokenType::RBrace) {
                self.next_token();
                return;
            }

            // If we reached a token that can start a statement, we can continue parsing. However,
            // our parsing functions require that for these tokens, the current token is the special
            // keyword token. So, we cannot advance the parser by one token, as that would set the
            // current token to the next token, and not the special keyword token. Instead, we return
            // from the function, and let the parsing function handle the current token.
            if matches!(
                token.r#type,
                TokenType::Let
                    | TokenType::Return
                    | TokenType::If
                    | TokenType::Else
                    | TokenType::Function
            ) {
                return;
            }

            token = self.lexer.next_token();
        }
    }

    /// The `next_token` method advances the parser by one token. This is done by calling `next_token`
    /// on the `lexer` field, and then setting `current_token` to `peek_token`, and then setting
    /// `peek_token` to the next token from the lexer.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// The `cur_token_is` method checks if the current token is of a given type.
    fn cur_token_is(&self, r#type: TokenType) -> bool {
        self.current_token.r#type == r#type
    }

    /// The `peek_token_is` method checks if the peek token is of a given type.
    fn peek_token_is(&self, r#type: TokenType) -> bool {
        self.peek_token.r#type == r#type
    }

    /// The `expect_peek` method checks if the peek token is of a given type, and if it is, it advances
    /// the parser by one token and returns true. If it is not, it returns false.
    fn expect_peek(&mut self, r#type: TokenType) -> ParseResult<()> {
        if self.peek_token_is(r#type) {
            self.next_token();
            Ok(())
        } else {
            Err(self.peek_error(r#type))
        }
    }

    /// The `peek_error` method creates a new `Error` with a given message, and adds it to the `errors`
    /// field.
    fn peek_error(&mut self, tt: TokenType) -> Error {
        Error::new(format!(
            "expected next token to be {}, got {} instead",
            tt, self.peek_token.r#type
        ))
    }
}
