//! Titania compiler.

use crate::ast::{src, wat};
use crate::error::*;
use crate::table::Table;
use crate::types::*;

// Result type for parsing functions.
pub type ResultCompile<T> = Result<T, Error>;

/// Translates a Titania AST to a WAT AST.
pub fn compile(module: &src::Module) -> ResultCompile<wat::Module> {
    let mut table_proc = Table::new();

    let name = module.name.clone();
    let mut funcs = Vec::new();

    for decl in module.decls.iter() {
        let func = compile_decl(&mut table_proc, &decl)?;
        funcs.push(func);
    }

    Ok(wat::Module { name, funcs })
}

fn compile_decl(table_proc: &mut Table<TypeProc>, decl: &src::Decl) -> ResultCompile<wat::Func> {
    match decl {
        src::Decl::Proc(decl_proc) => compile_proc(table_proc, decl_proc),
    }
}

fn compile_proc(
    table_proc: &mut Table<TypeProc>,
    decl_proc: &src::DeclProc,
) -> ResultCompile<wat::Func> {
    let src::DeclProc { name, line } = &decl_proc;

    // Make sure the proce name isn't being re-defined.
    if let Some(_) = table_proc.lookup(name) {
        return Error::name_redefinition(name, *line);
    }

    // Create an entry in the proc table.
    let t_proc = TypeProc::new(None);
    table_proc.push(name, t_proc);

    Ok(wat::Func { name: name.clone() })
}

#[cfg(test)]
mod tests {
    use crate::ast::src::builder::*;
    use crate::table::Table;

    use super::*;

    type ResultTest = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_module_empty() -> ResultTest {
        let module_name = "M";
        let module = BuilderModule::new().set_name(module_name).build();
        let module = compile(&module)?;
        assert_eq!(module.name, module_name);
        Ok(())
    }

    #[test]
    fn test_module_proc_empty() -> ResultTest {
        let module_name = "M";
        let proc_name = "P";
        let module = BuilderModule::new()
            .set_name(module_name)
            .add_decl(BuilderDeclProc::new().set_name(proc_name, 1).build_decl())
            .build();
        let module = compile(&module)?;
        assert_eq!(module.name, module_name);
        assert_eq!(module.funcs.len(), 1);
        Ok(())
    }

    #[test]
    fn test_compile_module_proc_name_redefinition() -> ResultTest {
        let mut builder_decl_proc = BuilderDeclProc::new();
        let module = BuilderModule::new()
            .set_name("M")
            .add_decl(builder_decl_proc.set_name("P", 2).build_decl())
            .add_decl(builder_decl_proc.set_name("P", 3).build_decl())
            .build();
        let compile_result = compile(&module);
        match compile_result {
            Err(Error {
                tag: ErrorTag::NameRedefinition(name),
                line,
            }) if name == "P" && line == 3 => Ok(()),
            _ => panic!("Expected name redefinition error."),
        }
    }

    #[test]
    fn test_compile_proc() -> ResultTest {
        let mut table_proc = Table::new();
        let proc_name = "P";
        let t_proc = TypeProc::new(None);
        let proc = BuilderDeclProc::new().set_name(proc_name, 1).build_decl();
        let func = compile_decl(&mut table_proc, &proc)?;
        assert_eq!(func.name, proc_name);
        assert_eq!(table_proc.lookup(proc_name), Some(&t_proc));
        Ok(())
    }
}
