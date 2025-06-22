//! Error handling.
use std::fmt;

use crate::scanner::TokenTag;

/// Enumerates all possible errors.
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorTag {
    /// Expected an identifier token tag, but got a different token tag.
    ExpectedIdentifier { got: TokenTag },

    /// Expected a token tag, but got a different token tag.
    ExpectedToken { expected: TokenTag, got: TokenTag },

    /// An unexpected character was encountered.
    UnexpectedCharacter(char),

    /// A comment was not terminated.
    UnterminatedComment,
}

impl fmt::Display for ErrorTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            ErrorTag::ExpectedIdentifier { got } => {
                return write!(f, "expected an identifier but got `{got}`");
            }
            ErrorTag::ExpectedToken { expected, got } => {
                return write!(f, "expected `{expected}` but got `{got}`");
            }
            ErrorTag::UnexpectedCharacter(c) => {
                return write!(f, "unexpected character `{c}`");
            }
            ErrorTag::UnterminatedComment => "unterminated comment",
        };
        write!(f, "{desc}")
    }
}

/// Represents an error in a source text.
#[derive(Debug)]
pub struct Error {
    /// What kind of error was encountered.
    pub tag: ErrorTag,

    /// On which line the error is located.
    pub line: usize,
}

impl Error {
    /// Constructs a new `Error` value.
    pub fn new(tag: ErrorTag, line: usize) -> Self {
        Self { tag, line }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error at line {}: {}", self.line, self.tag)
    }
}

impl std::error::Error for Error {}

/// If result is an error, returns the tag. Otherwise returns `None`.
#[cfg(test)]
pub fn error_tag<T>(result: Result<T, Error>) -> Option<ErrorTag> {
    match result {
        Err(Error { tag, .. }) => Some(tag),
        _ => None,
    }
}
