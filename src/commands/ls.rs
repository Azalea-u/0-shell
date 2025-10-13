// Built-in: ls (list directory contents)

use std::fs::{read_dir, DirEntry, Metadata};
use std::os::unix::fs::PermissionsExt;

use crate::behavior::shell::Shell;
use crate::behavior::tokenizer::Redirect;
use crate::core::redirect;

#[derive(Debug)]
pub struct Options {
    all: bool,
    long: bool,
    classify:bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            all: false,
            long: false,
            classify: false,
        }
    }
} 

pub fn run(_shell: &Shell, args: Vec<&str>, redirects: &[Redirect]) {
    let mut paths = Vec::new();
    let mut options = Options::default();
    
    for arg in args {
        match arg {
            "-a" => options.all = true,
            "-l" => options.long = true,
            "-F" => options.classify = true,

            "-la" | "-al" => {
                options.all = true;
                options.long = true;
            }
            "-lF" | "-Fl" => {
                options.long = true;
                options.classify = true;
            }
            "-aF" | "-Fa" => {
                options.all = true;
                options.classify = true;
            }

            "-laF" | "-lFa" | "-alF" | "-aFl" | "-Fla" | "-Fal" => {
                options.all = true;
                options.long = true;
                options.classify = true;
            }

            _ => {
                if arg.starts_with('-') {
                    eprintln!("ls: invalid option {}", arg);
                } else {
                    paths.push(arg.to_string());
                }
            }
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    for path in paths {
        list_directory(&path, &options, redirects);
    }
}

pub fn list_directory(path: &str, options: &Options, redirects: &[Redirect]) {
    match redirect::get_output_stream(redirects) {
        Ok(mut output) => {
            match read_dir(path) {
                Ok(entries) => {
                    let mut files: Vec<_> = entries.flatten().collect();
                    
                    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
                    
                    for entry in files {
                        let file_name = entry.file_name();                        
                        if !options.all && file_name.to_string_lossy().starts_with('.') {
                            continue;
                        }

                        let display_name = if options.classify {
                            classify_it(&entry)
                        } else {
                            file_name.to_string_lossy().to_string()
                        };

                        if options.long {
                            if let Ok(metadata) = entry.metadata() {
                                let size = metadata.len();
                                let file_type = file_type(&metadata);
                                let mode = metadata.permissions().mode();
                                
                                writeln!(output, "{}{} {:>8} {}", file_type,display_perm(mode), size, display_name).unwrap();
                            } else {
                                writeln!(output, "?---------        ? {}", display_name).unwrap();
                            }
                        } else {
                            write!(output, "{}  ", display_name).unwrap();
                        }
                    }
                    
                    if !options.long {
                        writeln!(output).unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("ls: cannot access '{}': {}", path, e);
                }
            }
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

pub fn classify_it(entry: &DirEntry) -> String {
    if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
            format!("{}/", entry.file_name().to_string_lossy().to_string())
        } else {
            entry.file_name().to_string_lossy().to_string()
        }
    }else {
        entry.file_name().to_string_lossy().to_string()
    }
}
 
 fn display_perm(mode: u32) -> String {
    let mut s = String::new();
    let flags = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];
    for (flag, c) in flags {
        s.push(if mode & flag != 0 { c } else { '-' });
    }
    s
}

fn file_type(metadata: &Metadata) -> char {
    use std::os::unix::fs::FileTypeExt;
    let file_type = metadata.file_type();
    
    if file_type.is_dir() { 'd' }
    else if file_type.is_file() { '-' }
    else if file_type.is_symlink() { 'l' }
    else if file_type.is_char_device() { 'c' }
    else if file_type.is_block_device() { 'b' }
    else if file_type.is_fifo() { 'p' }
    else if file_type.is_socket() { 's' }
    else { '?' }
}
