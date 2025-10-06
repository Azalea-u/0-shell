// Library root: re-exports and glue code

use std::io::{self, Write};

pub mod commands;
pub mod behavior;
pub mod utils;
pub mod error;

use behavior::shell::Shell;

pub fn run() {
    let mut shell = Shell::new();

    loop {
        print!("{}", shell.prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            break; // Ctrl+D
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "exit" if commands::exit::exit() => break,
            "cd" => commands::cd::run(&mut shell, args),
            _ => println!("{}: command not found", input),
        }

        shell.refresh();
    }
}
