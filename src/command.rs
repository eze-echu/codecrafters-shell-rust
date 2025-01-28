use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
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
                if param.is_empty() {
                    return Err("type: missing argument".into());
                }
                if !param.is_empty() && (param == "exit" || param == "echo" || param == "type" || param == "pwd" || param == "cd") {
                    Ok(Command {
                        execution: Box::new(move || println!("{} is a shell builtin", param)),
                    })
                } else {
                    let path_programs = Command::programs_on_path();
                    if path_programs.contains_key(param.as_str()) {
                        Ok(Command {
                            execution: Box::new(move || {
                                // println!("{:#?}", path_programs);
                                println!(
                                    "{} is {}",
                                    param,
                                    path_programs.get(param.as_str()).unwrap()
                                )
                            }),
                        })
                    } else {
                        Ok(Command {
                            execution: Box::new(move || println!("{}: not found", param)),
                        })
                    }
                }
            }
            "pwd" => {
                let real_path = fs::canonicalize(Path::new("."))
                    .expect("Failed to get current working directory");
                let str_path = real_path.into_os_string();
                Ok(Command {
                    execution: Box::new( move || {
                        println!("{}", str_path.to_str().unwrap());
                    })
                })
            }
            "cd" => {
                if param.split(' ').count() > 1 {
                    return Err("cd: too many arguments".into());
                }
                let path = PathBuf::from(&param);
                if path.try_exists()?{
                    Ok(Command {
                        execution: Box::new(move || {
                            std::env::set_current_dir(path).expect("Failed to change directory");
                        }),
                    })
                }
                else {
                    Err(format!("cd: {}: No such file or directory", param).into())
                }
            }
            _ => {
                let path_programs = Command::programs_on_path();
                if path_programs.contains_key(command) {
                    let command = command.trim().to_string();
                    Ok(Command {
                        execution: Box::new(move || {
                            let stdout = String::from_utf8(std::process::Command::new(command)
                                .args(param.split_ascii_whitespace())
                                .spawn()
                                .unwrap().wait_with_output().unwrap().stdout).unwrap();
                            print!("{}", stdout);
                        }),
                    })
                } else {
                    let command_name = command.to_string();
                    Ok(Command {
                        execution: Box::new(move || {
                            println!("{}: command not found", command_name)
                        }),
                    })
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
            .filter_map(|path| std::fs::read_dir(path).ok())
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
