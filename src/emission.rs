//! Code emission.

use crate::ast::wat::*;

pub fn emit_module(module: &Module) -> String {
    let indent = "    ";
    let mut code = String::new();

    code.push_str("(module $");
    code.push_str(&module.name);
    code.push_str("\n");

    for func in module.funcs.iter() {
        code.push_str(indent);
        code.push_str("(func $");
        code.push_str(&func.name);
        code.push_str("\n");
        code.push_str(indent);
        code.push_str(")\n");
    }

    for export in module.exports.iter() {
        code.push_str(indent);
        code.push_str("(export \"");
        code.push_str(&export.name);
        code.push_str("\" (func $");
        code.push_str(&export.name);
        code.push_str("))\n");
    }

    code.push_str(")\n");

    code
}
