use std::io::{self, Write};
use signal_hook::{consts::SIGINT, iterator::Signals};
use crate::commands;

pub fn run() {
    // Handle Ctrl+C in background thread
    let mut signals = Signals::new(&[SIGINT]).unwrap();
    std::thread::spawn(move || {
        for _ in signals.forever() {
            println!(); 
            print!("> ");
            io::stdout().flush().unwrap();
        }
    });

    // Main loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            println!();
            break;  // Ctrl+D
        }

        let input = input.trim();
        
        if input == "exit" && commands::exit::exit() {
            break;
        }
        
        println!("You typed: {}", input);
    }
}