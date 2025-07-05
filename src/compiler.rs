//! Titania compiler.

use crate::ast::src::DeclProc;
use crate::ast::{src, wat};
use crate::error::*;
use crate::table::Table;
use crate::types::*;

// Result type for parsing functions.
pub type ResultCompile<T> = Result<T, Error>;

/// Translates a Titania AST to a WAT AST.
pub fn compile(module: &src::Module) -> ResultCompile<wat::Module> {
    let mut table_type = create_default_type_table();
    let mut table_proc = Table::new();

    let name = module.name.clone();
    let mut funcs = Vec::new();
    let mut exports = Vec::new();

    for decl in module.decls.iter() {
        let (func, export) = compile_decl(&mut table_type, &mut table_proc, &decl)?;
        if let Some(export) = export {
            exports.push(export);
        }
        funcs.push(func);
    }

    Ok(wat::Module {
        name,
        funcs,
        exports,
    })
}

fn compile_decl(
    table_type: &Table<Type>,
    table_proc: &mut Table<TypeProc>,
    decl: &src::Decl,
) -> ResultCompile<(wat::Func, Option<wat::Export>)> {
    match decl {
        src::Decl::Proc(decl_proc) => compile_proc(table_type, table_proc, decl_proc),
    }
}

fn compile_proc(
    table_type: &Table<Type>,
    table_proc: &mut Table<TypeProc>,
    decl_proc: &src::DeclProc,
) -> ResultCompile<(wat::Func, Option<wat::Export>)> {
    // Make sure the proc name isn't being re-defined.
    if let Some(_) = table_proc.lookup(&decl_proc.name) {
        return Error::name_redefinition(&decl_proc.name, decl_proc.line);
    }

    let mut builder = wat::builder::BuilderFunc::new();
    builder.set_name(&decl_proc.name);

    let t_return = decl_proc
        .tid_return
        .as_ref()
        .map(|tid| lookup_type(&table_type, &tid))
        .transpose()?;
    let t_return_wat = t_return.as_ref().map(to_type_wat).transpose()?;
    builder.set_result(t_return_wat);
    table_proc.push(&decl_proc.name, TypeProc::new(t_return));

    let func = builder.build();
    let export = if decl_proc.export {
        Some(wat::Export {
            name: decl_proc.name.clone(),
        })
    } else {
        None
    };

    Ok((func, export))
}

/// Creates a type table with built-in types.
fn create_default_type_table() -> Table<Type> {
    let mut t = Table::new();
    t.push("INTEGER", Type::new_int());
    t
}

/// Lookup type associated with an given identifier.
fn lookup_type(table_type: &Table<Type>, tid: &str) -> ResultCompile<Type> {
    let Some(t) = table_type.lookup(tid) else {
        unimplemented!();
    };
    Ok(t.clone())
}

/// Convert a type to a WAT type.
fn to_type_wat(t: &Type) -> ResultCompile<wat::Type> {
    match t.tag() {
        TypeTag::Int => Ok(wat::Type::I32),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::src::builder::*;
    use crate::ast::wat::builder::*;
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
    fn test_module_proc_export() -> ResultTest {
        let module = BuilderModule::new()
            .set_name("M")
            .add_decl(
                BuilderDeclProc::new()
                    .set_name("P", 1)
                    .set_export(true)
                    .build_decl(),
            )
            .build();
        let module = compile(&module)?;
        assert_eq!(module.exports[0].name, "P");
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
        let mut table_type = create_default_type_table();
        let mut table_proc = Table::new();
        let proc_name = "P";
        let t_proc = TypeProc::new(None);
        let proc = BuilderDeclProc::new().set_name(proc_name, 1).build_decl();
        let (func, _) = compile_decl(&mut table_type, &mut table_proc, &proc)?;
        assert_eq!(func.name, proc_name);
        assert_eq!(table_proc.lookup(proc_name), Some(&t_proc));
        Ok(())
    }

    #[test]
    fn test_compile_proc_with_result_i32() -> ResultTest {
        let proc_name = "P";
        let proc = BuilderDeclProc::new()
            .set_name(proc_name, 1)
            .set_tid_return("INTEGER")
            .build();
        let func = BuilderFunc::new()
            .set_name(proc_name)
            .set_result(Some(wat::Type::I32))
            .build();
        let t_proc = TypeProc::new(Some(Type::new_int()));

        let mut table_type = create_default_type_table();
        let mut table_proc = Table::new();
        let (func_compiled, _) = compile_proc(&mut table_type, &mut table_proc, &proc)?;

        assert_eq!(func, func_compiled);
        assert_eq!(table_proc.lookup(proc_name), Some(&t_proc));

        Ok(())
    }
}
