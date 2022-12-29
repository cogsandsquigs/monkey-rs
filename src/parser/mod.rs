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

/// The parser for the Monkey programming language. It takes a `Lexer` and parses it into an AST.
pub struct Parser {
    /// The `lexer` field is the `Lexer` that the parser is parsing.
    lexer: Lexer,

    /// Any errors we encounter while parsing are stored in the `errors` field.
    errors: Vec<Error>,

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

            errors: vec![],

            current_token: Token::default(),
            peek_token: Token::default(),

            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Prime the parser by calling `next_token` twice.
        parser.next_token();
        parser.next_token();

        parser
    }

    /// Returns any and all errors that the parser has encountered so far during parsing.
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    /// Parses the input from the `Lexer` into an AST.
    pub fn parse_program(&mut self) -> Result<Program, &[Error]> {
        let mut program = Program { statements: vec![] };

        while self.current_token.r#type != TokenType::EOF {
            let stmt = self.parse_statement();

            if let Ok(stmt) = stmt {
                program.statements.push(stmt);
            }

            self.next_token();
        }

        if self.errors.is_empty() {
            Ok(program)
        } else {
            Err(&self.errors)
        }
    }
}

/// Private API for the `Parser` struct.
impl Parser {
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
    fn expect_peek(&mut self, r#type: TokenType) -> bool {
        if self.peek_token_is(r#type) {
            self.next_token();
            true
        } else {
            self.peek_error(r#type);
            false
        }
    }

    /// The `peek_error` method creates a new `Error` with a given message, and adds it to the `errors`
    /// field.
    fn peek_error(&mut self, tt: TokenType) {
        let error = Error::new(format!(
            "expected next token to be {}, got {} instead",
            tt, self.peek_token.r#type
        ));

        self.errors.push(error);
    }
}
