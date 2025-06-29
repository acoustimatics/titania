//! Abstract syntax tree (AST) types.

/// AST for source code.
pub mod src {
    /// A module.
    #[derive(Debug)]
    pub struct Module {
        /// The module's name.
        pub name: String,

        /// The module's declaration list.
        pub decls: Vec<Decl>,
    }

    /// All possible declarations.
    #[derive(Debug)]
    pub enum Decl {
        /// A procedure declaration.
        Proc(DeclProc),
    }

    /// A procedure declaration.
    #[derive(Debug)]
    pub struct DeclProc {
        /// The procedure's name.
        pub name: String,

        /// The line the procedure is defined on.
        pub line: usize,
    }

    pub mod builder {
        use std::mem;

        use super::*;

        pub struct BuilderModule {
            pub name: String,
            pub decls: Vec<Decl>,
        }

        impl BuilderModule {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    decls: Vec::new(),
                }
            }

            pub fn set_name(&mut self, name: &str) -> &mut Self {
                self.name = name.to_owned();
                self
            }

            pub fn add_decl(&mut self, decl: Decl) -> &mut Self {
                self.decls.push(decl);
                self
            }

            pub fn build(&mut self) -> Module {
                let name = mem::replace(&mut self.name, String::new());
                let decls = mem::replace(&mut self.decls, Vec::new());
                Module { name, decls }
            }
        }

        pub struct BuilderDeclProc {
            pub name: String,
            pub line: usize,
        }

        impl BuilderDeclProc {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    line: 0,
                }
            }

            pub fn set_name(&mut self, name: &str, line: usize) -> &mut Self {
                self.name = name.to_owned();
                self.line = line;
                self
            }

            pub fn build(&mut self) -> DeclProc {
                let name = mem::replace(&mut self.name, String::new());
                let line = mem::replace(&mut self.line, 0);
                DeclProc { name, line }
            }

            pub fn build_decl(&mut self) -> Decl {
                let decl_proc = self.build();
                Decl::Proc(decl_proc)
            }
        }
    }
}

/// WebAssembly text format AST.
pub mod wat {
    /// A WAT module.
    #[derive(Debug)]
    pub struct Module {
        /// The module's name.
        pub name: String,

        /// The module's functions.
        pub funcs: Vec<Func>,
    }

    /// A WAT function.
    #[derive(Debug)]
    pub struct Func {
        /// The function's name.
        pub name: String,
    }
}
