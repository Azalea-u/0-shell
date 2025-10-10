// Library root: re-exports and glue code
use std::io::{self, Write};

pub mod behavior;
pub mod commands;

use behavior::shell::Shell;
use behavior::parser;

pub fn run() {
    let mut shell = Shell::new();

    loop {
        let mut input = String::new();

        loop {
            print!("{}", shell.promt());
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if io::stdin().read_line(&mut line).unwrap() == 0 {
                println!();
                return; // Ctrl+D
            }

            input.push_str(&line);

            if parser::gate_closed(&input) {
                shell.gate_opened = false;
                break;
            } else {
                shell.gate_opened = true;
            }
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
            "cd" => commands::cd::run(&mut shell, args),
            "pwd" => commands::pwd::run(&mut shell, args),
            _ => println!("{}: command not found", cmd),
        }

        shell.refresh();
    }
}
