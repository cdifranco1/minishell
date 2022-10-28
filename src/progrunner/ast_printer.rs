use super::parser::*;

pub fn print_ast(expr: Expr) -> () {
    println!("{:#?}", expr);
}
