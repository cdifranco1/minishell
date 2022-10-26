use nix::unistd;
use std::process::{Child, Command, Stdio};

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

pub fn map_error<T, E>(res: Result<T, E>, message: &'static str) -> Result<T, &'static str> {
    res.map_err(|_| message)
}

pub fn cd(args: &Vec<String>) -> Result<Option<Child>, &'static str> {
    let maybe_arg = get_arg(args, 0);

    if let Some(arg) = maybe_arg {
        unistd::chdir(arg).map_err(|_| "cd command failed")?;
        Ok(None)
    } else {
        Ok(None)
    }
}

pub fn ls(args: &Vec<String>, maybe_stdout: Option<Stdio>) -> Result<Option<Child>, &'static str> {
    let maybe_arg = get_arg(args, 0);

    let cmd = if let Some(arg) = maybe_arg {
        Command::new("ls")
            .arg(arg)
            .stdout(maybe_stdout.unwrap())
            .spawn()
    } else {
        Command::new("ls").stdout(maybe_stdout.unwrap()).spawn()
    };

    map_error(cmd, "Could not perform ls").map(|res| Some(res))
}

pub fn echo(args: &Vec<String>, stdout: Option<Stdio>) -> Result<Option<Child>, &'static str> {
    let maybe_arg = get_arg(args, 0);

    let cmd = if let Some(arg) = maybe_arg {
        Command::new("echo")
            .arg(arg)
            .stdout(stdout.unwrap())
            .spawn()
    } else {
        Command::new("echo").stdout(stdout.unwrap()).spawn()
    };

    map_error(cmd, "Could not perform echo").map(|res| Some(res))
}

pub fn wc(
    args: &Vec<String>,
    stdin: Option<Stdio>,
    stdout: Option<Stdio>,
) -> Result<Option<Child>, &'static str> {
    let arg = get_arg(args, 0);

    let child = if let Some(a) = arg {
        Command::new("wc")
            .arg(a)
            .stdin(stdin.unwrap())
            .stdout(stdout.unwrap())
            .spawn()
    } else {
        Command::new("wc")
            .stdin(stdin.unwrap())
            .stdout(stdout.unwrap())
            .spawn()
    };

    map_error(child, "Could not perform wc").map(|res| Some(res))
}
