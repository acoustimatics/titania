//! Parser

use crate::ast::src::builder::*;
use crate::ast::src::*;
use crate::error::*;
use crate::scanner::*;

// Result type for parsing functions.
pub type ResultParse<T> = Result<T, Error>;

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
    pub fn module(&mut self) -> ResultParse<Module> {
        let mut builder_module = BuilderModule::new();

        // "module"
        self.expect(TokenTag::Module)?;

        // Id
        let (name, _) = self.expect_identifier()?;
        builder_module.set_name(&name);

        // ";"
        self.expect(TokenTag::Semicolon)?;

        // { Decl }
        while let Some(decl) = self.decl()? {
            builder_module.add_decl(decl);
        }

        // "end"
        self.expect(TokenTag::End)?;

        // "."
        self.expect(TokenTag::Dot)?;

        // EOF
        self.expect(TokenTag::Eof)?;

        Ok(builder_module.build())
    }

    /// Parses a declaration.
    pub fn decl(&mut self) -> ResultParse<Option<Decl>> {
        let decl = if self.is_match(TokenTag::Procedure)? {
            // Proc
            let proc = self.proc()?;

            // ";"
            self.expect(TokenTag::Semicolon)?;

            Some(Decl::Proc(proc))
        } else {
            None
        };

        Ok(decl)
    }

    /// Parses a procedure.
    pub fn proc(&mut self) -> ResultParse<DeclProc> {
        // "procedure" was previous token.

        // Id ["*"]
        let (name, line) = self.expect_identifier()?;
        let export = self.is_match(TokenTag::Star)?;

        // ";"
        self.expect(TokenTag::Semicolon)?;

        // "end"
        self.expect(TokenTag::End)?;

        let decl = BuilderDeclProc::new()
            .set_name(&name, line)
            .set_export(export)
            .build();
        Ok(decl)
    }

    /// Make sure the current token has the given tag, or else generate an error.
    fn expect(&mut self, expected: TokenTag) -> ResultParse<()> {
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
    fn expect_identifier(&mut self) -> ResultParse<(String, usize)> {
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
    fn is_match(&mut self, tag: TokenTag) -> ResultParse<bool> {
        if self.current.tag == tag {
            self.advance()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Sets current token to the next token in the source text.
    fn advance(&mut self) -> ResultParse<()> {
        self.current = self.scanner.next_token()?;
        Ok(())
    }

    /// Creates an error result for the current token.
    fn err_current<T>(&self, tag: ErrorTag) -> ResultParse<T> {
        Err(Error::new(tag, self.current.line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_module() -> ResultParse<()> {
        let mut parser = Parser::new("module M; end.")?;
        let module = parser.module()?;
        assert_eq!(module.name, "M");
        assert_eq!(module.decls.len(), 0);
        assert!(is_at_eof(&parser));
        Ok(())
    }

    #[test]
    fn test_module_procedure() -> ResultParse<()> {
        let mut parser = Parser::new("module M; procedure P; end; end.")?;
        let module = parser.module()?;
        assert_eq!(module.decls.len(), 1);
        assert!(is_at_eof(&parser));
        Ok(())
    }

    #[test]
    fn test_procedure_empty() -> ResultParse<()> {
        let mut parser = Parser::new("P; end")?;
        let decl_proc = parser.proc()?;
        assert_eq!(decl_proc.name, "P");
        assert_eq!(decl_proc.export, false);
        assert!(is_at_eof(&parser));
        Ok(())
    }

    #[test]
    fn test_procedure_export() -> ResultParse<()> {
        let mut parser = Parser::new("P*; end")?;
        let decl_proc = parser.proc()?;
        assert_eq!(decl_proc.name, "P");
        assert_eq!(decl_proc.export, true);
        assert!(is_at_eof(&parser));
        Ok(())
    }

    fn is_at_eof(parser: &Parser) -> bool {
        parser.current.tag == TokenTag::Eof
    }
}
