#[derive(Debug, Copy, Clone)]
pub enum Command {
    Exit,
    Unknown,
    Ls,
    Cd,
    Echo,
    Wc,
    Grep,
    Cat,
}

impl Command {
    pub fn try_from_string(command: String) -> Option<Command> {
        match command.trim() {
            "exit" => Some(Command::Exit),
            "ls" => Some(Command::Ls),
            "cd" => Some(Command::Cd),
            "echo" => Some(Command::Echo),
            "wc" => Some(Command::Wc),
            "grep" => Some(Command::Grep),
            "cat" => Some(Command::Cat),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Op {
    Pipe,
}

#[derive(Debug)]
pub enum Token {
    Operator(Op),
    Command(Command),
    Flag(String),
    Argument(String),
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match &self {
            Token::Operator(op) => match op {
                Op::Pipe => Token::Operator(Op::Pipe),
            },
            Token::Command(prog) => match prog {
                Command::Ls => Token::Command(Command::Ls),
                Command::Cd => Token::Command(Command::Cd),
                Command::Echo => Token::Command(Command::Echo),
                Command::Wc => Token::Command(Command::Wc),
                Command::Grep => Token::Command(Command::Grep),
                Command::Cat => Token::Command(Command::Cat),
                Command::Exit => Token::Command(Command::Exit),
                Command::Unknown => Token::Command(Command::Unknown),
            },
            Token::Flag(f) => Token::Flag(f.clone()),
            Token::Argument(arg) => Token::Argument(arg.clone()),
        }
    }
}

impl Token {
    pub fn try_from_string(curr: &Option<Token>, s: &String) -> Option<Token> {
        match s.trim() {
            "exit" => Some(Token::Command(Command::Exit)),
            "ls" => Some(Token::Command(Command::Ls)),
            "cd" => Some(Token::Command(Command::Cd)),
            "echo" => Some(Token::Command(Command::Echo)),
            "wc" => Some(Token::Command(Command::Wc)),
            "|" => Some(Token::Operator(Op::Pipe)),
            "grep" => Some(Token::Command(Command::Grep)),
            "cat" => Some(Token::Command(Command::Cat)),
            other if other.starts_with("-") => {
                match curr {
                    Some(Token::Command(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Argument(_)) => Some(Token::Flag(other.to_string())),
                    Some(Token::Flag(_)) => Some(Token::Flag(other.to_string())),
                    // should probably exit the Command and log something
                    Some(Token::Operator(_)) => None,
                    None => None,
                }
            }
            other => Some(Token::Argument(other.to_string())),
        }
    }
}

fn split(text: String) -> Vec<String> {
    let mut res = vec![];

    let mut buffer: String = String::new();
    let mut previous: Option<char> = None;

    let mut in_quote = false;

    for c in text.chars().into_iter() {
        if c == '\"' && !in_quote {
            in_quote = true;
            previous = Some(c);
            continue;
        }

        if c == '\"' && in_quote {
            in_quote = false;

            res.push(buffer.clone());
            buffer.clear();
            previous = Some(c);
            continue;
        }

        if c.is_whitespace() && in_quote {
            buffer.push(c);
            previous = Some(c);
        } else if c.is_whitespace() {
            match previous {
                Some(prev) => {
                    if prev.is_whitespace() || prev == '\"' {
                        continue;
                    } else {
                        res.push(buffer.clone());
                        buffer.clear();
                    }
                }
                None => {
                    previous = Some(c);
                }
            }
        } else if c.to_string().is_empty() {
            continue;
        } else {
            buffer.push(c);
            previous = Some(c);
        }
    }

    res
}

pub fn tokenize(text: String) -> Vec<Token> {
    let split_str = split(text);
    let mut current = None;

    split_str
        .into_iter()
        .map(|lexeme| {
            let result = Token::try_from_string(&current, &lexeme);
            current = result.clone();
            result
        })
        .flat_map(|value| value)
        .collect()
}

/*
impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.input.into_iter().next();

        if let Some(match_str) = next {
            let result = Token::try_from_string(&self.curr, &match_str.to_string());
            self.curr = result.clone();
            result
        } else {
            None
        }
    }
}
*/
