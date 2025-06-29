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

    /// A name previously defined was used in a definition.
    NameRedefinition(String),

    /// An unexpected character was encountered.
    UnexpectedCharacter(char),

    /// A comment was not terminated.
    UnterminatedComment,
}

impl fmt::Display for ErrorTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorTag::*;

        match self {
            ExpectedIdentifier { got } => {
                write!(f, "expected an identifier but got `{got}`")
            }
            ExpectedToken { expected, got } => {
                write!(f, "expected `{expected}` but got `{got}`")
            }
            NameRedefinition(name) => {
                write!(f, "name `{name}` was previously defined")
            }
            UnexpectedCharacter(c) => {
                write!(f, "unexpected character `{c}`")
            }
            UnterminatedComment => write!(f, "unterminated comment"),
        }
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

    pub fn name_redefinition<T>(name: &str, line: usize) -> Result<T, Self> {
        let name = name.to_owned();
        let tag = ErrorTag::NameRedefinition(name);
        let error = Self { tag, line };
        Err(error)
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
