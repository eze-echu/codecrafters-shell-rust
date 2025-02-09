use std::fs;
use std::str::FromStr;
use thiserror::Error;

mod type_func;
use type_func::*;

mod echo;
use echo::echo;

mod cd;
use cd::cd;

mod pwd;
use pwd::pwd;

const BUILTINS: &[&str] = &["type", "exit", "echo", "cd", "pwd"];

pub struct Command {
    execution: Box<dyn FnOnce()>,
}
#[derive(Debug, Error)]
enum CommandError {
    #[error("{command}: missing argument.")]
    MissingOnlyArgument { command: String },
    #[error("{command}: too many arguments.")]
    TooManyArguments { command: String },
    #[error("{command}: too few arguments.")]
    TooFewArguments { command: String },
    #[error("{command}: {destination}: No such file or directory.")]
    MissingFileOrDirectory {
        command: String,
        destination: String,
    },
    #[error("{command}: not found.")]
    NotFound { command: String },
}
impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, param) = s.split_at(s.find(' ').unwrap_or(s.len()));
        let param = param.trim().to_string();
        match command {
            "exit" => Ok(Command {
                execution: Box::new(move || {
                    std::process::exit(i32::from_str(param.as_str()).unwrap_or(0))
                }),
            }),
            "echo" => Ok(echo(param)),
            "type" => type_func_command(param),
            "pwd" => pwd(),
            "cd" => cd(param),
            _ => {
                let path_programs = Command::programs_on_path();
                if path_programs.contains_key(command) {
                    let command = command.trim().to_string();
                    Ok(Command {
                        execution: Box::new(move || {
                            let stdout = String::from_utf8(
                                std::process::Command::new(command)
                                    .args(param.split_ascii_whitespace())
                                    .spawn()
                                    .unwrap()
                                    .wait_with_output()
                                    .unwrap()
                                    .stdout,
                            )
                            .unwrap();
                            print!("{}", stdout);
                        }),
                    })
                } else {
                    Err(CommandError::NotFound {
                        command: command.to_string(),
                    }
                    .into())
                }
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
            .filter_map(|path| fs::read_dir(path).ok())
            .flatten()
            .for_each(|entry| {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file()
                    && !programs.contains_key(entry.file_name().to_string_lossy().as_ref())
                {
                    let file_name = entry.file_name().into_string().unwrap();
                    let file_path = entry.path().to_string_lossy().to_string();
                    programs.insert(file_name, file_path);
                }
            });
        programs
    }
}
