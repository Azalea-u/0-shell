use std::io::{self, Write};

pub mod behavior;
pub mod commands;
pub mod core;

use behavior::shell::Shell;
use behavior::parser;
use behavior::tokenizer;
use core::command;

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

        // Step 1: Basic parsing (quotes, escapes)
        let tokens = parser::parse_input(input);
        if tokens.is_empty() {
            continue;
        }

        // Step 2: Parse operators and redirections
        match tokenizer::tokens_parser(tokens) {
            Ok(command) => {
                command::execute(&mut shell, command);
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }

        shell.refresh();
    }
}
