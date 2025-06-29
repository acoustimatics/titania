//! Parser

use crate::ast::src::builder::*;
use crate::ast::src::*;
use crate::error::*;
use crate::scanner::*;

// Result type for parsing functions.
pub type ParseResult<T> = Result<T, Error>;

/// Holds the state of a parser.
pub struct Parser<'a> {
    /// A source text scanner.
    scanner: Scanner<'a>,

    /// Current token in the source code.
    current: Token,
}

impl<'a> Parser<'a> {
    /// Constructs a Parser for a source text.
    pub fn new(source: &'a str) -> Result<Parser<'a>, Error> {
        let mut scanner = Scanner::new(source);
        let current = scanner.next_token()?;
        Ok(Parser { scanner, current })
    }

    /// Parses a module.
    pub fn module(&mut self) -> ParseResult<Module> {
        let mut module_builder = BuilderModule::new();

        // "module"
        self.expect(TokenTag::Module)?;

        // Identifier
        let (name, _) = self.expect_identifier()?;
        module_builder.set_name(&name);

        // ";"
        self.expect(TokenTag::Semicolon)?;

        // { Declaration }
        while let Some(declaration) = self.declaration()? {
            module_builder.add_decl(declaration);
        }

        // "end"
        self.expect(TokenTag::End)?;

        // "."
        self.expect(TokenTag::Dot)?;

        // EOF
        self.expect(TokenTag::Eof)?;

        Ok(module_builder.build())
    }

    /// Parses a declaration.
    pub fn declaration(&mut self) -> ParseResult<Option<Decl>> {
        let declaration = if self.is_match(TokenTag::Procedure)? {
            // Procedure
            let procedure = self.procedure()?;

            // ";"
            self.expect(TokenTag::Semicolon)?;

            Some(procedure)
        } else {
            None
        };

        Ok(declaration)
    }

    /// Parses a procedure.
    pub fn procedure(&mut self) -> ParseResult<Decl> {
        // "procedure" was previous token.

        // Identifier
        let (name, line) = self.expect_identifier()?;

        // ";"
        self.expect(TokenTag::Semicolon)?;

        // "end"
        self.expect(TokenTag::End)?;

        let decl_proc = DeclProc { name, line };

        Ok(Decl::Proc(decl_proc))
    }

    /// Make sure the current token has the given tag, or else generate an error.
    fn expect(&mut self, expected: TokenTag) -> ParseResult<()> {
        if self.current.tag == expected {
            self.advance()?;
            Ok(())
        } else {
            self.err_current(ErrorTag::ExpectedToken {
                expected,
                got: self.current.tag.clone(),
            })
        }
    }

    /// If the current token is an identifier, return the identifier name and
    /// line number. Otherwise, return an error.
    fn expect_identifier(&mut self) -> ParseResult<(String, usize)> {
        match &self.current {
            Token {
                tag: TokenTag::Identifier(name),
                line,
            } => {
                let name = name.clone();
                let line = *line;
                self.advance()?;
                Ok((name, line))
            }
            _ => self.err_current(ErrorTag::ExpectedIdentifier {
                got: self.current.tag.clone(),
            }),
        }
    }

    /// If the current token matches the given tag, advance and return true.
    /// Otherwise, do nothing and return false.
    fn is_match(&mut self, tag: TokenTag) -> ParseResult<bool> {
        if self.current.tag == tag {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Sets current token to the next token in the source text.
    fn advance(&mut self) -> ParseResult<()> {
        self.current = self.scanner.next_token()?;
        Ok(())
    }

    /// Creates an error result for the current token.
    fn err_current<T>(&self, tag: ErrorTag) -> ParseResult<T> {
        Err(Error::new(tag, self.current.line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_module() -> ParseResult<()> {
        let mut parser = Parser::new("module M; end.")?;
        let module = parser.module()?;
        assert_eq!(module.name, "M");
        assert!(is_at_eof(&parser));
        Ok(())
    }

    #[test]
    fn test_module_procedure() -> ParseResult<()> {
        let mut parser = Parser::new("module M; procedure P; end; end.")?;
        let module = parser.module()?;
        assert_eq!(module.decls.len(), 1);
        assert!(is_at_eof(&parser));
        Ok(())
    }

    #[test]
    fn test_declaration_empty_procedure() -> ParseResult<()> {
        let mut parser = Parser::new("procedure P; end;")?;
        match parser.declaration()? {
            Some(Decl::Proc(decl_proc)) => {
                assert_eq!(decl_proc.name, "P");
            }
            decl => panic!("Expected a procedure but got {decl:?}"),
        }
        assert!(is_at_eof(&parser));
        Ok(())
    }

    fn is_at_eof(parser: &Parser) -> bool {
        parser.current.tag == TokenTag::Eof
    }
}
