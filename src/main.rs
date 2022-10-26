use std::error;
use std::io;
use std::io::Write;

use minishell::progrunner::run_program;

fn main() -> Result<(), Box<dyn error::Error>> {
    loop {
        print!("minishell > ");
        io::stdout().flush()?;

        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer)?;
        run_program(&buffer)?;

        buffer.clear();

        ()
    }
}
