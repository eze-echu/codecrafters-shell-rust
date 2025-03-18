use anyhow::Error;
use std::borrow::Cow;
use std::fmt::DebugList;
use std::fs;
use std::io::BufRead;
use std::ops::{Deref, DerefMut};
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
#[derive(Debug, Error, PartialEq, Eq)]
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

        let test_param = Self::parse_param(&param)?;

        //println!("{:#?}", test_param);
        match command {
            "exit" => Ok(Command {
                execution: Box::new(move || {
                    std::process::exit(i32::from_str(param.as_str()).unwrap_or(0))
                }),
            }),
            "echo" => Ok(echo(test_param.join(""))),
            "type" => type_func_command(test_param[0].to_owned()),
            "pwd" => pwd(),
            "cd" => cd(test_param.join("")),
            _ => {
                let path_programs = Command::programs_on_path();
                if path_programs.contains_key(command) {
                    let command = command.trim().to_string();
                    Ok(Command {
                        execution: Box::new(move || {
                            let stdout = String::from_utf8(
                                std::process::Command::new(command)
                                    .args(test_param)
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
    fn parse_param(param: &str) -> Result<Vec<String>, anyhow::Error> {
        let mut quotations = Quotations {
            single_quote: false,
            double_quote: false,
            braces: false,
            parenthesis: false,
            backtick: false,
            escaped: false,
            buffer: String::default(),
        };

        let mut groups: Vec<String> = vec![];

        let trimmed_param = param.trim();
        if trimmed_param.is_empty() {
            return Ok(vec![])
        }
        for i in 0..trimmed_param.len() {
            let param_char = trimmed_param.chars().nth(i).unwrap();

            // TODO: Make function to handle checking
            match param_char {
                '\'' => {
                    if quotations.escaped {
                        quotations.buffer_push(param_char);
                        quotations.escaped = false;
                    }
                    else if quotations.single_quote {
                        quotations.single_quote = false;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                        // quotations.buffer_push(param_char);
                    } else if quotations.is_already_inside_quotations() {
                        quotations.buffer_push(param_char);
                    } else {
                        quotations.single_quote = true;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                    }
                }
                '"' => {
                    if quotations.escaped {
                        quotations.buffer_push(param_char);
                        quotations.escaped = false;
                    }
                    else if quotations.double_quote {
                        quotations.double_quote = false;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                        //quotations.buffer_push(param_char);
                    } else if quotations.is_already_inside_quotations() {
                        quotations.buffer.push(param_char);
                    } else {
                        quotations.double_quote = true;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                    }
                }
                '`' => {
                    if quotations.escaped {
                        quotations.buffer_push(param_char);
                        quotations.escaped = false;
                    }
                    else if quotations.backtick {
                        quotations.single_quote = false;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                        //quotations.buffer_push(param_char);
                    } else if quotations.is_already_inside_quotations() {
                        quotations.buffer_push(param_char);
                    } else {
                        quotations.backtick = true;
                        groups.push(quotations.buffer());
                        quotations.buffer_clear();
                    }
                }
                '\\' => {
                    if quotations.single_quote || (quotations.escaped && quotations.is_already_inside_quotations()){
                        quotations.buffer_push(param_char);
                    }
                    quotations.escaped = true;
                }
                _ => {
                    quotations.buffer_push(param_char);
                }
            }
        }
        if !quotations.buffer.is_empty() {
            let a = quotations.buffer.split_whitespace().collect::<Vec<&str>>().join(" ");
            groups.push(a);
        }
        Ok(groups.into_iter().filter(|s| !s.is_empty()).collect::<Vec<String>>())
    }
}

struct Quotations {
    pub single_quote: bool,
    pub double_quote: bool,
    pub braces: bool,
    pub parenthesis: bool,
    pub backtick: bool,
    pub escaped: bool,
    buffer: String,
}

impl Quotations {
    fn is_already_inside_quotations(&self) -> bool {
        self.single_quote || self.double_quote || self.braces || self.parenthesis || self.backtick
    }

    fn buffer_push(&mut self, param_char: char) {
        self.buffer.push(param_char);
    }

    fn buffer_clear(&mut self){
        self.buffer.clear();
    }

    fn buffer(&self) -> String {
        self.buffer.clone()
    }
}
