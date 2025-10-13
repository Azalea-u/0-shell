// Command Execution
use crate::behavior::shell::Shell;
use crate::behavior::tokenizer::Command;
use crate::commands::*;

pub fn execute(shell: &mut Shell, cmd: Command) {
    let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
    match cmd.name.as_str() {
        "exit" => std::process::exit(0),
        "cd" => {
            cd::run(shell, args, &cmd.redirects);
        }
        "pwd" => {
            pwd::run(shell, args, &cmd.redirects);
        }
        "echo" => {
            echo::run(args, &cmd.redirects);
        }
        "ls" => {
            ls::run(shell, args, &cmd.redirects);
        }
        _ => println!("{}: command not found", cmd.name),
    }
}
