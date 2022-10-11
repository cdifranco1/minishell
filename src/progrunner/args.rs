use std::str::{Chars, SplitWhitespace};


#[derive(Debug)]
pub enum Program {
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
    Command(Program),
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
                Program::Ls => Token::Command(Program::Ls),
                Program::Cd => Token::Command(Program::Cd),
                Program::Echo => Token::Command(Program::Echo),
                Program::Exit => Token::Command(Program::Exit),
                Program::Unknown => Token::Command(Program::Unknown)
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
            "exit" => Some(Token::Command(Program::Exit)),
            "ls" => Some(Token::Command(Program::Ls)),
            "cd" => Some(Token::Command(Program::Cd)),
            "echo" => Some(Token::Command(Program::Echo)),
            "|" => Some(Token::Operator(Op::Pipe)),
            other if other.starts_with("-") => {
               match curr {
                   Some(Token::Command(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Argument(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Flag(_)) => Some(Token::Flag(other.to_string())),
                    // should probably exit the program and log something
                    Some(Token::Operator(_)) => None,
                    None => None
               }

            }
            other => Some(Token::Argument(other.to_string()))
        }
    }
}


impl Program {
    pub fn from_string(command: String) -> Program {
        match command.trim() {
            "exit" => Program::Exit,
            "ls" => Program::Ls,
            "cd" => Program::Cd,
            "echo" => Program::Echo,
            _ => Program::Unknown,
        }
    }

    pub fn try_from_string(command: String) -> Option<Program> {
        match Program::from_string(command) {
            Program::Unknown => None,
            other => Some(other)
        }
    }
}

#[derive(Debug)]
pub struct Args {
    pub program: Program,
    pub arguments: Vec<String>,
}

impl Args {
    pub fn build(buffer: &String) -> Result<Args, &'static str> {
        let mut split_args = buffer.split(" ");

        let program = match split_args.next() {
            Some(command) => Some(Program::from_string(command.to_string())),
            None => None,
        };

        let args: Vec<String> = split_args.map(|x| x.to_string()).collect();

        if let Some(prog) = program {
            Ok(Args {
                program: prog,
                arguments: args,
            })
        } else {
            Err("Unable to build args")
        }
    }
}

pub struct Scanner<'a> {
    input: SplitWhitespace<'a>,
    curr: Option<Token>,
}


impl<'a> Scanner<'a> {
    fn new(text: &'a String) -> Self {
        Scanner { input: text.split_whitespace(), curr: None }
    }

    /*
    fn gather_string(curr: &char, iter: &mut Chars<'a>) -> String {
        let s: String = iter.take_while(|x| !x.is_whitespace()).collect();
        let mut match_str = String::new();
        match_str.push_str(&curr.to_string()); // this is okay because returning a new value
        match_str.push_str(&s);
        match_str
    }
    */
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
