use crate::behavior::shell::Shell;
use std::path::PathBuf;
use std::env;

pub fn run(shell: &mut Shell, args: &[&str]) {

    // Determine target path
    let target = if args.is_empty() || args[0] == "~" {
        // cd or cd ~ → home directory
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else if args[0].starts_with("~") && args[0].len() > 1 {
        // cd ~username
        let user = &args[0][1..];
        format!("/home/{}", user)
    } else if args[0] == "-" {
        // cd -
        if let Some(old) = &shell.old_cwd {
            old.display().to_string()
        } else {
            eprintln!("cd: no previous directory");
            return;
        }
    } else {
        args[0].to_string()
    };

    let mut new_path = PathBuf::from(&target);
    if !new_path.is_absolute() && args.get(0) != Some(&"-") {
        new_path = shell.cwd.join(new_path);
    }

    if new_path.is_dir() {
        if let Ok(abs_path) = new_path.canonicalize() {
            shell.update_dir(abs_path);
        } else {
            eprintln!("cd: invalid path: {}", target);
        }
    } else {
        eprintln!("cd: no such directory: {}", target);
    }
}
