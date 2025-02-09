use crate::command::Command;

pub fn echo(message: String) -> Command {
    Command {
        execution: Box::new(move || println!("{}", message)),
    }
}
