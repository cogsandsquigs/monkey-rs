mod tests;

use crate::token::{Token, TokenType};

// The `Lexer` struct preforms lexical analysis on the input string, and returns
// tokens that can be used by the parser.
pub struct Lexer {
    /// The input string that is being lexed. This is a `Vec<char>` instead of a `&str` because
    /// it is easier to work with (for example, retrieving a character at an index).
    input: Vec<char>,

    /// The position of the current character we are lexing in the input string. Note that in
    /// the original Monkey implementation, this field is called `position`, but I renamed it
    /// to `current_position` to make it more clear what this is used for.
    current_position: usize,

    /// The position of the next character we are lexing in the input string. Note that in the
    /// original Monkey implementation, this field is called `read_position`, but I renamed it
    /// to `next_position` to make it more clear what this is used for.
    next_position: usize,

    /// The current character we are lexing in the input string. This is used as a "storage
    /// space" to keep the current character in, so that we don't have to do annoying things to
    /// get the current character from the input string.
    ch: char,
}

/// Public API for the `Lexer` struct.
impl Lexer {
    /// Creates a new `Lexer` from the given input string. This also "primes" the lexer by calling
    /// `read_char` once, so that the `ch` field is set to the first character in the input string.
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            // Convert the input string to a `Vec<char>` so that we can easily get characters.
            input: input.chars().collect(),

            // `current_position` is 0 because the first character in the input string is at index 0.
            current_position: 0,

            // `next_position` is 0 so that when `read_char` is called, it will set `current_position`
            // to 0, and increment this to 1.
            next_position: 0,

            // `ch` is set to `\0` because we don't know what the next character is. This will be
            // set to said character when `read_char` is called.
            ch: '\0',
        };

        // "prime" the lexer by calling `read_char` once
        lexer.read_char();

        lexer
    }

    /// Returns the next token in the input string.
    pub fn next_token<'a>(&mut self) -> Token<'a> {
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch),
            '+' => Token::new(TokenType::Plus, self.ch),
            ',' => Token::new(TokenType::Comma, self.ch),
            ';' => Token::new(TokenType::Semicolon, self.ch),
            '(' => Token::new(TokenType::LParen, self.ch),
            ')' => Token::new(TokenType::RParen, self.ch),
            '{' => Token::new(TokenType::LBrace, self.ch),
            '}' => Token::new(TokenType::RBrace, self.ch),
            '\0' => Token::new(TokenType::Eof, "".to_string()),
            _ => todo!("Implement the rest of the lexer!"),
        };

        // Update the lexer's state to the next character in the input string.
        self.read_char();

        token
    }
}

/// Private API for the `Lexer` struct.
impl Lexer {
    /// Reads the next character from the input string, and stores it in the `ch` field. Also
    /// updates the `current_position` and `next_position` fields, and returns the character
    /// that was read, so that it can be used in the calling function. Note that if we are at
    /// the end of the input string, this function will return `\0`, and not update the
    /// `current_position` or `next_position` fields.
    fn read_char(&mut self) -> char {
        // Bounds checking.
        if self.next_position >= self.input.len() {
            self.ch = '\0';

            // We don't need to update `current_position` or `next_position` here, because
            // we are at the end of the input string.
        } else {
            self.ch = self.input[self.next_position];

            // Update `current_position` and `next_position` to point to the next character.
            self.current_position = self.next_position;
            self.next_position += 1;
        }

        self.ch
    }
}
