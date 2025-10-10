// Library root: re-exports and glue code
use std::io::{self, Write};

pub mod behavior;
pub mod commands;

use behavior::shell::Shell;
use behavior::parser;

pub fn run() {
    let mut shell = Shell::new();

    loop {
        print!("{}", shell.promt());
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

        let tokens = parser::parse_input(input);
        if tokens.is_empty() {
            continue;
        }

        let cmd = tokens[0].as_str();
        let args: Vec<&str> = tokens[1..].iter().map(|s| s.as_str()).collect();

        match cmd {
            "exit" => break,
            _ => println!("{}: command not found", cmd),
        }

        shell.refresh();
    }
}