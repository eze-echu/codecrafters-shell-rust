use crate::command::{Command, CommandError, BUILTINS};
use anyhow::{Error, Result};

pub fn type_func_command(param: String) -> Result<Command> {
    if param.is_empty() {
        return Err(Error::new(CommandError::MissingOnlyArgument {
            command: "type".into(),
        }));
    }
    if BUILTINS.contains(&param.as_str()) {
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
