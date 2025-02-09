use crate::command::{Command, CommandError};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn cd(destination: String) -> Result<Command> {
    if destination.is_empty() {
        return Err(CommandError::MissingOnlyArgument {
            command: "cd".into(),
        }
        .into());
    }
    if destination.split(' ').count() > 1 {
        return Err(CommandError::TooManyArguments {
            command: "cd".into(),
        }
        .into());
    }
    let str_path = destination.replace(
        "~",
        std::env::var("HOME")
            .with_context(|| "Unable to find HOME variable, ~ is not available")?
            .as_str(),
    );
    let path = PathBuf::from(&str_path);
    if path.try_exists()? {
        Ok(Command {
            execution: Box::new(move || {
                std::env::set_current_dir(path).expect("Failed to change directory");
            }),
        })
    } else {
        Err(CommandError::MissingFileOrDirectory {
            command: "cd".into(),
            destination,
        }
        .into())
    }
}
