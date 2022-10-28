pub mod ast_printer;
pub mod parser;
pub mod programs;
pub mod scanner;

use parser::*;
use std::process::Stdio;

pub fn run_program(input: &String) -> Result<(), &'static str> {
    let ast = parse(input.to_string())?;
    let res = ast.eval(Some(Stdio::inherit()), Some(Stdio::inherit()));

    let wait = res.map(|c| match c {
        Some(mut child) => Some(child.wait()),
        None => None,
    });

    match wait {
        Ok(_) => Ok(()),
        Err(_) => Err("Failure"),
    }
}
