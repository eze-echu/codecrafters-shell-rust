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
                } else if Command::programs_on_path().contains_key(param.as_str()) {
                    Ok(Command {
                        execution: Box::new(move || {
                            println!(
                                "{} is {}",
                                param,
                                Command::programs_on_path().get(param.as_str()).unwrap()
                            )
                        }),
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
    fn programs_on_path() -> std::collections::HashMap<String, String> {
        let mut programs = std::collections::HashMap::new();
        std::env::var("PATH")
            .unwrap_or_else(|_| "/bin:/usr/bin".to_string())
            .split(':')
            .filter_map(|path| std::fs::read_dir(path).ok())
            .flatten()
            .for_each(|entry| {
                let entry = entry.unwrap();
                let file_name = entry.file_name().into_string().unwrap();
                let file_path = entry.path().to_string_lossy().to_string();
                programs.insert(file_name, file_path);
            });
        programs
    }
}
