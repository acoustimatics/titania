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
        Proc(Proc),
    }

    /// A procedure declaration.
    #[derive(Debug)]
    pub struct Proc {
        /// The procedure's name.
        pub name: String,

        /// The line the procedure is defined on.
        pub line: usize,

        /// Whether the procedure is exported.
        pub export: bool,

        /// Return type identifier.
        pub tid_return: Option<String>,
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

        pub struct BuilderProc {
            name: String,
            line: usize,
            export: bool,
            tid_return: Option<String>,
        }

        impl BuilderProc {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    line: 0,
                    export: false,
                    tid_return: None,
                }
            }

            pub fn set_name(&mut self, name: &str, line: usize) -> &mut Self {
                self.name = name.to_owned();
                self.line = line;
                self
            }

            pub fn set_export(&mut self, export: bool) -> &mut Self {
                self.export = export;
                self
            }

            pub fn set_tid_return(&mut self, tid_return: &str) -> &mut Self {
                self.tid_return = Some(tid_return.to_owned());
                self
            }

            pub fn build(&mut self) -> Proc {
                let name = mem::replace(&mut self.name, String::new());
                let line = mem::replace(&mut self.line, 0);
                let export = mem::replace(&mut self.export, false);
                let tid_return = mem::replace(&mut self.tid_return, None);
                Proc {
                    name,
                    line,
                    export,
                    tid_return,
                }
            }

            #[cfg(test)]
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

        /// The module's exports.
        pub exports: Vec<Export>,
    }

    /// A WAT function.
    #[derive(Debug, PartialEq)]
    pub struct Func {
        /// The function's name.
        pub name: String,

        /// The function's result.
        pub result: Option<Type>,
    }

    /// Represents an export S-expression.
    #[derive(Debug, PartialEq)]
    pub struct Export {
        /// The export's name.
        pub name: String,
    }

    /// WAT types.
    #[derive(Debug, PartialEq)]
    pub enum Type {
        /// The `i32` type.
        I32,
    }

    pub mod builder {
        use std::mem;

        use super::*;

        pub struct BuilderFunc {
            name: String,
            result: Option<Type>,
        }

        impl BuilderFunc {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    result: None,
                }
            }

            pub fn set_name(&mut self, name: &str) -> &mut Self {
                self.name = name.to_owned();
                self
            }

            pub fn set_result(&mut self, result: Option<Type>) -> &mut Self {
                self.result = result;
                self
            }

            pub fn build(&mut self) -> Func {
                let name = mem::replace(&mut self.name, String::new());
                let result = mem::replace(&mut self.result, None);
                Func { name, result }
            }
        }
    }
}
