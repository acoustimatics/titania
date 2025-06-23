//! Abstract syntax tree (AST) types.

/// AST for source code.
pub mod src {
    /// A module.
    #[derive(Debug)]
    pub struct Module {
        /// The module's name.
        pub name: String,

        /// The module's declaration list.
        pub declarations: Vec<Declaration>,
    }

    /// All possible declarations.
    #[derive(Debug)]
    pub enum Declaration {
        /// A procedure declaration.
        Procedure {
            /// The procedure's name.
            name: String,
        },
    }

    pub mod builder {
        use std::mem;

        use super::*;

        pub struct ModuleBuilder {
            pub name: String,
            pub declarations: Vec<Declaration>,
        }

        impl ModuleBuilder {
            pub fn new() -> Self {
                Self {
                    name: String::new(),
                    declarations: Vec::new(),
                }
            }

            pub fn set_name(&mut self, name: String) -> &mut Self {
                self.name = name;
                self
            }

            pub fn add_declaration(&mut self, declaration: Declaration) -> &mut Self {
                self.declarations.push(declaration);
                self
            }

            pub fn build(&mut self) -> Module {
                let name = mem::replace(&mut self.name, String::new());
                let declarations = mem::replace(&mut self.declarations, Vec::new());
                Module {
                    name,
                    declarations,
                }
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
