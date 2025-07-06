//! Lexical analysis for Titania

use std::fmt;
use std::str::Chars;

use crate::error::*;

/// Represents a token's type in a source text.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenTag {
    /// The `BEGIN` keyword.
    Begin,

    /// A `:`.
    Colon,

    /// A `.`
    Dot,

    /// The `END` keyword.
    End,

    /// Represents the end of the source text.
    Eof,

    /// A sequence of letters or digits that is not a keyword.
    Identifier(String),

    /// A sequence of digits.
    Integer(String),

    /// The `MODULE` keyword.
    Module,

    /// The `PROCEDURE` keyword.
    Procedure,

    /// The `RETURN` keyword.
    Return,

    /// A `;`.
    Semicolon,

    /// A '*'.
    Star,
}

impl fmt::Display for TokenTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenTag::*;

        let token_str = match self {
            Begin => "BEGIN",
            Colon => ":",
            Dot => ".",
            Eof => "EOF",
            End => "END",
            Identifier(id) => {
                return write!(f, "identifier({id})");
            }
            Integer(n) => {
                return write!(f, "integer({n})");
            }
            Module => "MODULE",
            Procedure => "PROCEDURE",
            Return => "Return",
            Semicolon => ";",
            Star => "*",
        };

        write!(f, "{token_str}")
    }
}

/// A token from a source text.
#[derive(Clone)]
pub struct Token {
    /// The token's type.
    pub tag: TokenTag,

    /// The line in the source text on which the token starts.
    pub line: usize,
}

impl Token {
    /// Constructs a new Token.
    pub fn new(tag: TokenTag, line: usize) -> Self {
        Self { tag, line }
    }
}

/// Represents an object which converts a source text into a stream of tokens.
pub struct Scanner<'a> {
    /// A character by character iterator over the source text.
    chars: Chars<'a>,

    /// Current character in the source text.
    current: Option<char>,

    /// Next character in the source text.
    next: Option<char>,

    /// The line the current character is on in the source text.
    line: usize,
}

impl<'a> Scanner<'a> {
    /// Constructs a scanner that is ready to produce tokens from a given
    /// source text.
    pub fn new(source: &'a str) -> Scanner<'a> {
        let mut scanner = Self {
            chars: source.chars(),
            current: None,
            next: None,
            line: 1,
        };
        scanner.advance();
        scanner.advance();
        scanner
    }

    /// Attempt to get the next token in the source text.
    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace_comments()?;

        match self.current {
            Some(c) if is_alpha(c) => self.identifier(),
            Some(c) if is_digit(c) => self.number(),
            _ => self.symbol(),
        }
    }

    /// Skips all white space characters and comments before the next token.
    fn skip_whitespace_comments(&mut self) -> Result<(), Error> {
        let mut in_comment = false;
        loop {
            match (self.current, self.next) {
                (Some('('), Some('*')) if !in_comment => {
                    in_comment = true;
                    self.advance();
                    self.advance();
                }

                (Some('*'), Some(')')) if in_comment => {
                    in_comment = false;
                    self.advance();
                    self.advance();
                }

                (Some(c), _) if is_whitespace(c) || in_comment => {
                    self.advance();
                }

                (None, _) if in_comment => {
                    return Err(Error::new(ErrorTag::UnterminatedComment, self.line));
                }

                _ => return Ok(()),
            }
        }
    }

    /// Scans an identifier token assuming that current is a valid starting
    /// character for an identifier.
    fn identifier(&mut self) -> Result<Token, Error> {
        use TokenTag::*;

        let line = self.line;

        let mut lexeme = String::new();
        loop {
            match self.current {
                Some(c) if is_alpha(c) || is_digit(c) => {
                    lexeme.push(c);
                    self.advance();
                }
                _ => break,
            }
        }

        let tag = match lexeme.as_ref() {
            "BEGIN" => Begin,
            "END" => End,
            "MODULE" => Module,
            "PROCEDURE" => Procedure,
            "RETURN" => Return,
            _ => Identifier(lexeme),
        };

        Ok(Token::new(tag, line))
    }

    /// Scans a number token assuming that current is digit.
    fn number(&mut self) -> Result<Token, Error> {
        let line = self.line;

        let mut lexeme = String::new();
        loop {
            match self.current {
                Some(c) if is_digit(c) => {
                    lexeme.push(c);
                    self.advance();
                }
                _ => break,
            }
        }

        Ok(Token::new(TokenTag::Integer(lexeme), line))
    }

    /// Scans a symbol token and end of file.
    fn symbol(&mut self) -> Result<Token, Error> {
        use TokenTag::*;

        let line = self.line;

        let tag = match self.current {
            None => Eof,
            Some(':') => Colon,
            Some('.') => Dot,
            Some(';') => Semicolon,
            Some('*') => Star,
            Some(c) => return Err(Error::new(ErrorTag::UnexpectedCharacter(c), self.line)),
        };

        self.advance();

        Ok(Token::new(tag, line))
    }

    /// Advances current to the next character in the source text.
    fn advance(&mut self) {
        if let Some('\n') = self.current {
            self.line += 1;
        }
        self.current = self.next;
        self.next = self.chars.next();
    }
}

/// Determines if a given character is alphabetic.
fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Determines if a given character is a digit.
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Determines if a given character is whitespace.
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::error_tag;

    fn identifier_tag(id: &str) -> TokenTag {
        TokenTag::Identifier(String::from(id))
    }

    fn integer_tag(n: &str) -> TokenTag {
        TokenTag::Integer(String::from(n))
    }

    fn next_tag(scanner: &mut Scanner) -> Result<TokenTag, Error> {
        Ok(scanner.next_token()?.tag)
    }

    #[test]
    fn test_next_token_empty() -> Result<(), Error> {
        let mut scanner = Scanner::new("");
        assert_eq!(next_tag(&mut scanner)?, TokenTag::Eof);
        Ok(())
    }

    #[test]
    fn test_skip_whitespace_comment() -> Result<(), Error> {
        let mut scanner = Scanner::new(" \t(* a comment \r\n*)\r\n  id");
        assert_eq!(next_tag(&mut scanner)?, identifier_tag("id"));
        Ok(())
    }

    #[test]
    fn test_next_token_unclosed_comment() {
        let mut scanner = Scanner::new("(**");
        assert_eq!(
            error_tag(scanner.next_token()),
            Some(ErrorTag::UnterminatedComment)
        );
    }

    #[test]
    fn test_next_token_identifier() -> Result<(), Error> {
        let mut scanner = Scanner::new("a anIdentifier abc123");
        assert_eq!(next_tag(&mut scanner)?, identifier_tag("a"));
        assert_eq!(next_tag(&mut scanner)?, identifier_tag("anIdentifier"));
        assert_eq!(next_tag(&mut scanner)?, identifier_tag("abc123"));
        Ok(())
    }

    #[test]
    fn test_next_token() -> Result<(), Error> {
        let mut scanner = Scanner::new("1 1234");
        assert_eq!(next_tag(&mut scanner)?, integer_tag("1"));
        assert_eq!(next_tag(&mut scanner)?, integer_tag("1234"));
        assert_eq!(next_tag(&mut scanner)?, TokenTag::Eof);
        Ok(())
    }

    #[test]
    fn test_next_token_keywords() -> Result<(), Error> {
        use TokenTag::*;

        let mut scanner = Scanner::new("BEGIN END MODULE PROCEDURE RETURN");
        assert_eq!(next_tag(&mut scanner)?, Begin);
        assert_eq!(next_tag(&mut scanner)?, End);
        assert_eq!(next_tag(&mut scanner)?, Module);
        assert_eq!(next_tag(&mut scanner)?, Procedure);
        assert_eq!(next_tag(&mut scanner)?, Return);
        assert_eq!(next_tag(&mut scanner)?, Eof);
        Ok(())
    }

    #[test]
    fn test_next_token_symbol() -> Result<(), Error> {
        use TokenTag::*;

        let mut scanner = Scanner::new(": . ; *");
        assert_eq!(next_tag(&mut scanner)?, Colon);
        assert_eq!(next_tag(&mut scanner)?, Dot);
        assert_eq!(next_tag(&mut scanner)?, Semicolon);
        assert_eq!(next_tag(&mut scanner)?, Star);
        assert_eq!(next_tag(&mut scanner)?, Eof);
        Ok(())
    }
}
