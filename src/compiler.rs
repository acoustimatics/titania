//! Titania compiler.

use crate::ast::{src, wat};

/// Translates a Titania AST to a WAT AST.
pub fn compile(module: &src::Module) -> wat::Module {
    let name = module.name.clone();

    for declaration in module.declarations.iter() {
        compile_declaration(&declaration);
    }

    wat::Module { name }
}

pub fn compile_declaration(declaration: &src::Declaration) {
}

#[cfg(test)]
mod tests {
    use crate::ast::src::builder::*;
    use crate::parser::*;

    use super::*;

    #[test]
    fn test_empty_module() -> ParseResult<()> {
        let module_name = "M";
        let module = ModuleBuilder::new().set_name(module_name.to_owned()).build();
        let module = compile(&module);
        assert_eq!(module.name, module_name);
        Ok(())
    }
}
