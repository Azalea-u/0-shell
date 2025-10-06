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

        if input == "exit" && commands::exit::exit() {
            break;
        }

        shell.refresh();

        println!("{}: command not found", input);
    }
}
