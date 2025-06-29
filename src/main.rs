mod ast;
mod compiler;
mod emission;
mod error;
mod parser;
mod scanner;
mod table;
mod types;

use std::env;
use std::fs;
use std::io::Write;

use crate::compiler::compile;
use crate::emission::emit_module;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: titania path");
        return;
    }

    match compile_file(&args[1]) {
        Ok(_) => (),
        Err(e) => eprintln!("error: {e}"),
    }
}

fn compile_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n# SOURCE");
    let source = fs::read_to_string(path)?;
    println!("{source}");

    println!("\n# PARSED");
    let mut parser = Parser::new(&source)?;
    let module = parser.module()?;
    println!("{:?}", module);

    println!("\n# COMPILED");
    let module = compile(&module)?;
    println!("{:?}", module);

    println!("\n# EMISSION");
    let code = emit_module(&module);
    println!("{code}");

    let wat_path = format!("{}.wat", module.name);
    let mut file = fs::File::create(wat_path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
