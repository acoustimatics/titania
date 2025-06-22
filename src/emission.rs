//! Code emission.

use crate::ast::wat::*;

pub fn emit_module(module: &Module) -> String {
    let mut code = String::new();

    code.push_str(&format!("(module ${}\n", module.name));

    code.push_str(")\n");

    code
}
