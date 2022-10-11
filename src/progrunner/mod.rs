pub mod args;
pub mod programs;

use args::*;
use programs::*;

pub fn run_program(input: &String) -> Result<(), &'static str> {
    let args = Args::build(&input)?;

    tokenize(input);

    println!("{:?}", args);

    Ok(())
}
