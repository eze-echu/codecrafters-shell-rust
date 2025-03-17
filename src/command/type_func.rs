use crate::command::{Command, CommandError, BUILTINS};
use anyhow::{Error, Result};

pub fn type_func_command(param: String) -> Result<Command> {
    if param.is_empty() {
        return Err(CommandError::MissingOnlyArgument {
            command: "type".into(),
        }.into());
    }
    if param.split(" ").count() != 1 {
        return Err(CommandError::TooManyArguments {
            command: "type".into(),
        }.into())
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

#[cfg(test)]
mod tests {
    use crate::command::CommandError;
    use crate::command::echo::echo;
    use crate::command::type_func::type_func_command;

    #[test]
    fn err_missing_argument() {
        let func = type_func_command(String::from(""));
        assert!(func.is_err());
        assert_eq!(func.err().unwrap().to_string(), "type: missing argument.");
    }
    
    #[test]
    fn err_multiple_arguments() {
        let func = type_func_command(String::from("arg1 arg2"));
        assert!(func.is_err());
        assert_eq!(func.err().unwrap().to_string(), "type: too many arguments.");
    }
    
    #[test]
    fn builtin_command() {
        let built_command = type_func_command(String::from("cd"));
        assert!(built_command.is_ok());
    }

}