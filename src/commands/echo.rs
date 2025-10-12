// Built-in: echo (print arguments)
use crate::core::redirect;

pub fn run(args: Vec<&str>, redirects: &[crate::behavior::tokenizer::Redirect]) {
    let mut decypher = true;
    let mut args = args;

    if !args.is_empty() && args[0] == "-E" {
        decypher = false;
        args = args[1..].to_vec();
    } else if !args.is_empty() && args[0] == "-e" {
        decypher = true;
        args = args[1..].to_vec();
    }

    // Get the appropriate output stream (stdout or file)
    match redirect::get_output_stream(redirects) {
        Ok(mut output) => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    write!(output, " ").unwrap();
                }
                
                let processed_arg = if decypher {
                    decyphering(arg)
                } else {
                    arg.to_string()
                };
                
                write!(output, "{}", processed_arg).unwrap();
            }
            writeln!(output).unwrap();
        }
        Err(e) => {
            eprintln!("echo: redirection error: {}", e);
        }
    }
}

pub fn decyphering(s: &str) -> String {
    s.replace("\\a", "\x07")
     .replace("\\b", "\x08")
     .replace("\\e", "\x1B")
     .replace("\\f", "\x0C")
     .replace("\\n", "\n")
     .replace("\\r", "\r")
     .replace("\\t", "\t")
     .replace("\\v", "\x0B")
     .replace("\\\\", "\\")
}
