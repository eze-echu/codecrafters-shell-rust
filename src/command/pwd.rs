use crate::command::Command;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn pwd() -> Result<Command> {
    let real_path =
        fs::canonicalize(Path::new(".")).expect("Failed to get current working directory");
    let str_path = real_path.into_os_string();
    Ok(Command {
        execution: Box::new(move || {
            println!("{}", str_path.to_str().unwrap());
        }),
    })
}
