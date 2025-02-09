mod command;
use command::*;

use anyhow::Context;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        let _ = stdin
            .read_line(&mut input)
            .with_context(|| "failed to read input");
        match Command::from_str(input.trim()) {
            Ok(command) => command.execute(),
            Err(e) => eprintln!("{}", e),
        }
        // let separated_input: Vec<&str> = input.split_whitespace().collect();
        // match separated_input[0] {
        //     "exit" => {
        //         Command::Echo(separated_input[1..].iter().map(|s| s.to_string()).collect());
        //         std::process::exit(0);
        //     }
        //     "echo" => {
        //         println!("{}", separated_input[1..].join(" "));
        //     }
        //     _ => {
        //         println!("{}: command not found", input.trim());
        //     }
        // }
    }
}
