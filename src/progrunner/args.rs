use std::str::{SplitWhitespace};


#[derive(Debug)]
pub enum Command {
    Exit,
    Unknown,
    Ls,
    Cd,
    Echo,
}

#[derive(Debug)]
pub enum Op {
    Pipe
}

#[derive(Debug)]
pub enum Token {
    Operator(Op),
    Command(Command),
    Flag(String),
    Argument(String)
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match &self {
            Token::Operator(op) => match op {
                Op::Pipe => Token::Operator(Op::Pipe)
            }
            Token::Command(prog) => match prog {
                Command::Ls => Token::Command(Command::Ls),
                Command::Cd => Token::Command(Command::Cd),
                Command::Echo => Token::Command(Command::Echo),
                Command::Exit => Token::Command(Command::Exit),
                Command::Unknown => Token::Command(Command::Unknown)
            }
            Token::Flag(f) => Token::Flag(f.clone()),
            Token::Argument(arg) => Token::Argument(arg.clone())
        }
    }
}

impl Token {
    pub fn try_from_string(curr: &Option<Token>, s: &String) -> Option<Token> {
        println!("Current {:?}", curr);
        println!("Match String {:?}", s);
        match s.trim() {
            "exit" => Some(Token::Command(Command::Exit)),
            "ls" => Some(Token::Command(Command::Ls)),
            "cd" => Some(Token::Command(Command::Cd)),
            "echo" => Some(Token::Command(Command::Echo)),
            "|" => Some(Token::Operator(Op::Pipe)),
            other if other.starts_with("-") => {
               match curr {
                   Some(Token::Command(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Argument(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Flag(_)) => Some(Token::Flag(other.to_string())),
                    // should probably exit the Command and log something
                    Some(Token::Operator(_)) => None,
                    None => None
               }

            }
            other => Some(Token::Argument(other.to_string()))
        }
    }
}

// "Expression" types
//  Have a "Program" as a type of expression, which consists of a command and it's associated
//  arguments. Equivalent to functions.
//
//

pub enum Expr {
    Program,
    Binary
}

#[derive(Debug)]
pub struct Program {
    pub command: Command,
    pub arguments: Vec<String>,
}

impl Program {
    pub fn build(buffer: &String) -> Result<Self, &'static str> {
        let mut split_args = buffer.split(" ");

        let command = match split_args.next() {
            Some(cmd) => Some(Command::from_string(cmd.to_string())),
            None => None,
        };

        let args: Vec<String> = split_args.map(|x| x.to_string()).collect();

        if let Some(cmd) = command {
            Ok(Program {
                command: cmd,
                arguments: args,
            })
        } else {
            Err("Unable to build args")
        }
    }
}

pub struct Binary {
    left: Box<Expr>,
    right: Box<Expr>,
    operator: Token
}



pub struct Scanner<'a> {
    input: SplitWhitespace<'a>,
    curr: Option<Token>,
}


impl<'a> Scanner<'a> {
    fn new(text: &'a String) -> Self {
        Scanner { input: text.split_whitespace(), curr: None }
    }
}


impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.input.next();

        if let Some(match_str) = next {
            let result = Token::try_from_string(&self.curr, &match_str.to_string());
            self.curr = result.clone();
            result
        } else {
            None
        }
        
    }
}

pub fn tokenize<'a>(buffer: &'a String) -> () {
    let scanner = Scanner::new(buffer);
    let tokens: Vec<Token> = scanner.into_iter().collect();

    println!("{:?}", tokens);
}


// Scan through list of tokens
// If encounter a Command, gather arguments, then construct a "Program"
// If encounter an operator (currently only "|"), 

// Different types of expressions should have different AST builds
//  -- Only rules we need to implement are for operators:
//  -- Currently only pipe op
//
// command    -> COMMAND_LOOKUP FLAGS ARGUMENTS 
//
// expression -> command expression command
// pipe       -> expression "|" expression



