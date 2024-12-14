use std::error::Error;
use std::str::FromStr;

pub struct Command {
    execution: Box<dyn FnOnce()>,
}
impl FromStr for Command {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, param) = s.split_at(s.find(' ').unwrap_or(s.len()));
        let param = param.trim().to_string();
        match command {
            "exit" => Ok(Command {
                execution: Box::new(move || {
                    std::process::exit(i32::from_str(param.as_str()).unwrap_or(0))
                }),
            }),
            "echo" => Ok(Command {
                execution: Box::new(move || println!("{}", param)),
            }),
            "type" => {
                if param.is_empty() || param != "exit" && param != "echo" && param != "type" {
                    Ok(Command {
                        execution: Box::new(move || println!("{}: not found", param)),
                    })
                } else {
                    Ok(Command {
                        execution: Box::new(move || println!("{} is a shell builtin", param)),
                    })
                }
            }
            _ => {
                let command_name = command.to_string();
                Ok(Command {
                    execution: Box::new(move || println!("{}: command not found", command_name)),
                })
            }
        }
    }
}
impl Command {
    pub fn execute(self) {
        (self.execution)();
    }
}
