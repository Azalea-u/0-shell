// Built-in: cd
use crate::behavior::shell::Shell;
use std::path::PathBuf;
use std::env;
use std::io;

pub fn run(shell: &mut Shell, args: Vec<&str>) {
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
            println!("{}", shell.l_cwd.display());
            return;
        }
    } else {
        args[0].to_string()
    };

    let new_path = PathBuf::from(&target);
    match shell.update_dir(new_path) {
        Ok(()) => {},
        Err(e) => {
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("cd: permission denied for {}", target);
                }
                io::ErrorKind::NotFound => {
                    eprintln!("cd: no such directory {}", target);
                }
                _ => {
                    eprintln!("cd: {}: {}", target, e);
                }
            }
        }
    }

}
