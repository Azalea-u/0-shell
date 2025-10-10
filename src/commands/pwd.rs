// Built-in: pwd (print working directory)
use crate::behavior::shell::Shell;

pub fn run(shell: &Shell, args: Vec<&str>) {
    // Default to logical (-L)
    let flag = args.get(0).copied().unwrap_or("-L");

    match flag {
        "-L" => println!("{}", shell.l_cwd.display()),
        "-P" => println!("{}", shell.p_cwd.display()),
        _ => {
            eprintln!("pwd: bad option: {}", flag);
        }
    }
}
