// Built-in: echo (print arguments)
pub fn run(args: Vec<&str>) {
    let mut decypher = true;
    let mut args = args;

    if !args.is_empty() && args[0] == "-E" {
        decypher = false;
        args = args[1..].to_vec();
    } else if !args.is_empty() && args[0] == "-e" {
        decypher = true;
        args = args[1..].to_vec();
    }

    for arg in args {
        let mut decyphered = arg.to_string();
        if decypher {
            decyphered = decyphering(arg)
        }
        print!("{} ", decyphered)
    }
    println!()
}

pub fn decyphering(s: &str) -> String{
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
