#[derive(Debug, Clone)]
pub enum Command {
    Ls,
    Cat(String),
    Ps,
    Open(String),
    Exit,
    Unknown(String),
}

pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Command {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Command::Unknown("".to_string());
        }

        match parts[0] {
            "ls" => Command::Ls,
            "cat" => {
                if parts.len() > 1 {
                    Command::Cat(parts[1].to_string())
                } else {
                    Command::Unknown("cat needs a file".to_string())
                }
            },
            "ps" => Command::Ps,
            "open" => {
                if parts.len() > 1 {
                    Command::Open(parts[1].to_string())
                } else {
                    Command::Unknown("open needs a file".to_string())
                }
            },
            "exit" | "quit" => Command::Exit,
            cmd => Command::Unknown(cmd.to_string()),
        }
    }
}
