use crate::{ast::Program, lexer::Lexer, token::Token};

/// The parser for the Monkey programming language. It takes a `Lexer` and parses it into an AST.
pub struct Parser {
    /// The `lexer` field is the `Lexer` that the parser is parsing.
    lexer: Lexer,

    /// The `current_token` field is the current token that the parser is looking at.
    current_token: Token,

    /// The `peek_token` field is the next token that the parser is looking at.
    peek_token: Token,
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
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    /// Parses the input from the `Lexer` into an AST.
    pub fn parse_program(&mut self) -> Program {
        todo!()
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
}
