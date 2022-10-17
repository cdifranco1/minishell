use nix::unistd;
use std::io;
use std::process::Command;

use crate::progrunner::args::Program;

pub fn get_arg(args: &Vec<String>, index: usize) -> Option<&str> {
    if args.len() > index {
        let arg = &*args[index].trim();
        if arg.is_empty() {
            None
        } else {
            Some(arg)
        }
    } else {
        None
    }
}

pub fn cd(program: Program) -> Result<(), &'static str> {
    let maybe_arg = get_arg(&program.arguments, 0);

    if let Some(arg) = maybe_arg {
        unistd::chdir(arg).map_err(|_| "cd command failed")
    } else {
        Ok(())
    }
}

pub fn ls(program: Program) -> Result<(), &'static str> {
    let maybe_arg = get_arg(&program.arguments, 0);

    let exit_status = if let Some(arg) = maybe_arg {
        Command::new("ls").arg(arg).status()
    } else {
        Command::new("ls").status()
    };

    match exit_status {
        Ok(_) => Ok(()),
        Err(_) => Err("could not execute ls command"),
    }
}

pub fn map_result<'a, E>(result: Result<(), E>, error_message: &'a str) -> Result<(), &'a str> {
    result.map_err(|_| error_message)
} 

pub fn echo(program: Program) -> Result<(), &'static str> {
    use io::Write;

    let maybe_arg = get_arg(&program.arguments, 0);

    match maybe_arg {
        None => map_result(io::stdout().write_all(b""), "some error message"),
        Some(arg) => {
            let mut writable = arg.to_string().replace("\"", "");
            writable.push_str("\n");

            let bytes = writable.into_bytes();
            map_result(io::stdout().write_all(&bytes), "other error message")
        }
    }
}


