use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::prelude::OsStrExt;
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

use crate::quotations::parse_quotes;

const BUILTINS: &[&str] = &["type", "exit", "echo", "cd", "pwd"];

pub struct Command {
    execution: Box<dyn FnOnce()>,
}
#[derive(Debug, Error, PartialEq, Eq)]
enum CommandError {
    #[error("{command}: missing argument.")]
    MissingOnlyArgument { command: String },
    #[error("{command}: too many arguments.")]
    TooManyArguments { command: String },
    #[error("{command}: too few arguments.")]
    TooFewArguments { command: String },
    #[error("{command}: {destination}: No such file or directory")]
    MissingFileOrDirectory {
        command: String,
        destination: String,
    },
    #[error("{command}: command not found")]
    CommandNotFound { command: String },
    #[error("{command}: parsing error")]
    ParsingError { command: String },
}
impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, param) = s.split_at(s.find(' ').unwrap_or(s.len()));
        let param = parse_quotes(param.trim());
        match command {
            "exit" => Ok(Command {
                execution: Box::new(move || {
                    std::process::exit(i32::from_str(param.as_str()).unwrap_or(0))
                }),
            }),
            "echo" => Ok(echo(param)),
            "type" => type_func_command(param.trim().to_string()),
            "pwd" => pwd(),
            "cd" => cd(param.trim().to_string()),
            _ => {
                if Self::binary_exists_on_path(command) {
                    let command = command.trim().to_string();
                    Ok(Command {
                        execution: Box::new(move || {
                            let stdout = String::from_utf8(
                                std::process::Command::new(command)
                                    .args(vec![param])
                                    .spawn()
                                    .unwrap()
                                    .wait_with_output()
                                    .unwrap()
                                    .stdout,
                            )
                            .unwrap();
                            println!("{stdout}");
                        }),
                    })
                } else {
                    Err(CommandError::CommandNotFound {
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
    #[cfg(target_os = "linux")]
    /// # source_path
    ///
    /// ## Description
    /// Linux-specific function to retrieve all executable programs available in the system's PATH.
    /// ## Returns
    /// - binaries_found: HashMap<String, String>, where the key is the program name and the value is its full path.
    fn source_path() -> std::collections::HashMap<String, String> {
        let mut binaries_found = std::collections::HashMap::new();
        std::env::var("PATH")
            .unwrap_or_else(|_| "/bin:/usr/bin".to_string())
            .split(':')
            .filter_map(|path| fs::read_dir(path).ok())
            .flatten()
            .for_each(|entry| {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file()
                    && !binaries_found.contains_key(entry.file_name().to_string_lossy().as_ref())
                {
                    let file_name = entry.file_name().into_string().unwrap();
                    let file_path = entry.path().to_string_lossy().to_string();
                    binaries_found.insert(file_name, file_path);
                }
            });
        binaries_found
    }
    fn binary_exists_on_path(bin: &str) -> bool {
        let program_exists = if cfg!(target_os = "linux") {
            let path_programs = Command::source_path();
            path_programs.contains_key(bin)
        } else if cfg!(target_os = "windows") {
            // On Windows, we can use the `where` command to check for executables
            let path = std::env::var_os("PATH");
            if let Some(item) = path {
                item.to_string_lossy().find(bin).is_some()
            } else {
                false
            }
        } else {
            false
        };
        program_exists
    }
}
