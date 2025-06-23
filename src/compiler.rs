//! Titania compiler.

use crate::ast::{src, wat};

/// Translates a Titania AST to a WAT AST.
pub fn compile(module: &src::Module) -> wat::Module {
    let name = module.name.clone();
    let mut funcs = Vec::new();

    for declaration in module.declarations.iter() {
        let func = compile_declaration(&declaration);
        funcs.push(func);
    }

    wat::Module { name, funcs }
}

fn compile_declaration(declaration: &src::Declaration) -> wat::Func {
    match declaration {
        src::Declaration::Procedure { name } => {
            wat::Func { name: name.clone() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::src::*;
    use crate::ast::src::builder::*;

    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_empty_module() -> TestResult {
        let module_name = "M";
        let module = ModuleBuilder::new().set_name(module_name.to_owned()).build();
        let module = compile(&module);
        assert_eq!(module.name, module_name);
        Ok(())
    }

    #[test]
    fn test_empty_module_empty_proc() -> TestResult {
        let module_name = "M";
        let proc_name = "P";
        let module = ModuleBuilder::new()
            .set_name(module_name.to_owned())
            .add_declaration(Declaration::Procedure { name: proc_name.to_owned() })
            .build();
        let module = compile(&module);
        assert_eq!(module.name, module_name);
        assert_eq!(module.funcs.len(), 1);
        Ok(())
    }

    #[test]
    fn test_compile_proc() -> TestResult {
        let proc_name = "P";
        let proc = Declaration::Procedure { name: proc_name.to_owned() };
        let func = compile_declaration(&proc);
        assert_eq!(func.name, proc_name);
        Ok(())
    }
}
