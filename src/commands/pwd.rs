// Built-in: pwd (print working directory)
use crate::behavior::shell::Shell;
use crate::core::redirect;

pub fn run(shell: &Shell, args: Vec<&str>, redirects: &[crate::behavior::tokenizer::Redirect]) {
    // Default to logical (-L)
    let flag = args.get(0).copied().unwrap_or("-L");

    match redirect::get_output_stream(redirects) {
        Ok(mut output) => {
            match flag {
                "-L" => {
                    if let Err(e) = writeln!(output, "{}", shell.l_cwd.display()) {
                        eprintln!("pwd: output error: {}", e);
                    }
                }
                "-P" => {
                    if let Err(e) = writeln!(output, "{}", shell.p_cwd.display()) {
                        eprintln!("pwd: output error: {}", e);
                    }
                }
                _ => {
                    eprintln!("pwd: bad option: {}", flag);
                }
            }
        }
        Err(e) => {
            eprintln!("pwd: redirection error: {}", e);
        }
    }
}
