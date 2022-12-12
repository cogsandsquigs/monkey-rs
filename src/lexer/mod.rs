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
        // Skip whitespace characters.
        self.skip_whitespace();

        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch),
            '+' => Token::new(TokenType::Plus, self.ch),
            '-' => Token::new(TokenType::Minus, self.ch),
            '!' => Token::new(TokenType::Bang, self.ch),
            '*' => Token::new(TokenType::Star, self.ch),
            '/' => Token::new(TokenType::Slash, self.ch),
            '<' => Token::new(TokenType::Lt, self.ch),
            '>' => Token::new(TokenType::Gt, self.ch),
            ',' => Token::new(TokenType::Comma, self.ch),
            ';' => Token::new(TokenType::Semicolon, self.ch),
            '(' => Token::new(TokenType::LParen, self.ch),
            ')' => Token::new(TokenType::RParen, self.ch),
            '{' => Token::new(TokenType::LBrace, self.ch),
            '}' => Token::new(TokenType::RBrace, self.ch),
            '\0' => Token::new(TokenType::Eof, "".to_string()),

            // The nice thing about rust is that we can match only if the character satisfies
            // some arbitrary constraint. In this case, we are matching if the character is
            // a letter or an underscore. Returning here because we don't need to call `read_char`
            // again, as we already did that in the `read_identifier` function, at the end of the
            // loop.
            s if s.is_alphabetic() || s == '_' => return Token::from_ident(self.read_identifier()),

            // Parse integers. Returning here because we don't need to call `read_char` again, as we
            // already did that in the `read_number` function, at the end of the loop.
            s if s.is_ascii_digit() => return Token::new(TokenType::Int, self.read_number()),

            _ => Token::new(TokenType::Illegal, self.ch),
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

    /// Skips whitespace characters from the input string. This is used when we encounter a
    /// whitespace character, because that means we are lexing whitespace.
    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    /// Reads an identifier from the input string, and returns it as a `String`. This is used
    /// when we encounter a character that is a letter or an underscore, because that means we
    /// are lexing an identifier or keyword. It expects that `ch` is a letter or an underscore.
    fn read_identifier(&mut self) -> String {
        // Get the position of the first character in the identifier.
        let position = self.current_position;

        // Keep reading characters until we encounter a character that is not a letter, digit,
        // or underscore.
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }

        // Get the identifier from the input string.
        self.input[position..self.current_position].iter().collect()
    }

    /// Reads a number from the input string, and returns it as a `String`. This is used when
    /// we encounter a character that is a digit, because that means we are lexing a number.
    /// It expects that `ch` is a digit. Note that if you use this function, you cannot call
    /// `read_char` again, because this function already does that at the end of the loop.
    fn read_number(&mut self) -> String {
        // Get the position of the first character in the number.
        let position = self.current_position;

        // Keep reading characters until we encounter a character that is not a digit.
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        // Get the number from the input string.
        self.input[position..self.current_position].iter().collect()
    }
}
