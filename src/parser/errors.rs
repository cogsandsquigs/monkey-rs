use std::fmt::Display;

/// Custom error type for the parser. Note that the original implementation of the parser in the book
/// uses a plain string for the error message, but I think it's better to use a specific error type
/// so that we can add more information to it later. Also, it makes the code more readable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub message: String,
}

impl Error {
    /// Creates a new `Error` with the given `message`.
    pub fn new<S: ToString>(message: S) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
