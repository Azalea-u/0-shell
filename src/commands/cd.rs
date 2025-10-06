use crate::behavior::shell::Shell;

pub fn run(shell: &mut Shell, args: &[&str]) {
    if args.is_empty() {
        eprintln!("cd: missing argument");
        return;
    }

    let path = args[0];
    let new_path = if path.starts_with('/') {
        std::path::PathBuf::from(path)
    } else {
        shell.cwd.join(path)
    };

    if new_path.is_dir() {
        if let Ok(abs_path) = new_path.canonicalize() {
            if let Err(err) = std::env::set_current_dir(&abs_path) {
                eprintln!("cd: failed to change directory: {}", err);
                return;
            }
            shell.cwd = abs_path;
        } else {
            eprintln!("cd: invalid path: {}", path);
        }
    } else {
        eprintln!("cd: no such directory: {}", path);
    }
}
