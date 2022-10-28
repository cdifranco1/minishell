use super::programs;
use super::scanner::*;
use std::iter::Peekable;
use std::process::{Child, Stdio};

// can update this struct to accept references
// to avoid cloning
#[derive(Debug)]
pub struct Program {
    pub command: Command,
    pub arguments: Vec<String>,
}

// need to bifurcate built-ins vs. executables
impl Program {
    fn execute(
        &self,
        stdin: Option<Stdio>,
        stdout: Option<Stdio>,
    ) -> Result<Option<Child>, &'static str> {
        match self.command {
            Command::Ls => programs::ls(&self.arguments, stdout),
            Command::Wc => programs::wc(&self.arguments, stdin, stdout),
            Command::Grep => programs::grep(&self.arguments, stdin, stdout),
            Command::Cat => programs::cat(&self.arguments, stdout),
            Command::Echo => programs::echo(&self.arguments, stdout),
            Command::Cd => programs::cd(&self.arguments),
            Command::Exit => Err("Exiting shell..."),
            Command::Unknown => {
                println!("Unknown command");
                Ok(None)
            }
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Op,
}

impl Binary {
    // recursive function that connects pipes of child processes
    fn eval(
        &self,
        stdin: Option<Stdio>,
        stdout: Option<Stdio>,
    ) -> Result<Option<Child>, &'static str> {
        match self.operator {
            Op::Pipe => {
                let left = self.left.eval(stdin, Some(Stdio::piped()));

                let right_in: Option<Stdio> = match left {
                    Ok(Some(child)) => Some(child.stdout.unwrap().into()),
                    _ => None,
                };

                self.right.eval(right_in, stdout)
            }
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),   // this is a non-terminal expression
    Program(Program), // this is a terminal expression
}

impl Expr {
    // This will be kicked off with the parents stdin and stdout as Stdio::inherit
    pub fn eval(
        &self,
        read_f: Option<Stdio>,
        write_f: Option<Stdio>,
    ) -> Result<Option<Child>, &'static str> {
        match self {
            // maybe spawning a process;
            Expr::Program(prog) => prog.execute(read_f, write_f),
            Expr::Binary(bin) => bin.eval(read_f, write_f),
        }
    }
}

pub fn parse_program<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Program, &'static str> {
    if let Some(Token::Command(command)) = tokens.peek() {
        tokens.next();
        let mut arguments: Vec<String> = vec![];

        while let Some(Token::Flag(flag)) = tokens.peek() {
            tokens.next();
            arguments.push(flag.clone());
        }

        while let Some(Token::Argument(arg)) = tokens.peek() {
            tokens.next();
            arguments.push(arg.clone());
        }

        Ok(Program {
            command: command.clone(),
            arguments,
        })
    } else {
        Err("No command found")
    }
}

pub fn parse_pipe<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
) -> Result<Expr, &'static str> {
    let program = parse_program(tokens)?;

    match tokens.next() {
        Some(Token::Operator(Op::Pipe)) => Ok(Expr::Binary(Binary {
            left: Box::new(Expr::Program(program)),
            right: Box::new(parse_pipe(tokens)?),
            operator: Op::Pipe,
        })),
        _ => Ok(Expr::Program(program)),
    }
}

// Different types of expressions should have different AST builds
//  -- Only rules we need to implement are for operators (Currently only pipe op)
//
// command    -> COMMAND_LOOKUP FLAGS ARGUMENTS
//
// expression -> pipe;
// pipe       -> program ("|" pipe)* | program;
// program    -> command flags args;    **all terminal

pub fn parse(buffer: String) -> Result<Expr, &'static str> {
    let tokens = tokenize(buffer);

    parse_pipe(&mut tokens.iter().peekable())
}
