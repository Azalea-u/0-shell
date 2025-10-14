// Built-in: ls (list directory contents)

use std::fs::{read_dir, DirEntry, Metadata};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};

use chrono::{DateTime, Duration, Local};

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
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => options.all = true,
                    'l' => options.long = true,
                    'F' => options.classify = true,
                    _ => {eprintln!("ls: invalid option -{}", c);
                        return;},
                }
            }
        } else {
            paths.push(arg.to_string());
        }
    }

    if paths.is_empty() {
        paths.push(".".to_string());
    }

    let show_headers = paths.len() > 1;
    for (i, path) in paths.iter().enumerate() {
        if show_headers {
            if i > 0 {
                println!(); // blank line between outputs
            }
            println!("{}:", path);
        }

        list_directory(path, &options, redirects);
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
                                let file_type = file_type(&metadata);
                                let n_link = metadata.nlink();
                                let (user, group) = get_user_group(&metadata);
                                let last_updated = get_modified_time(&metadata);
                                let mode = metadata.permissions().mode();
                                let size = if file_type == 'c' || file_type == 'b' {
                                    get_device_numbers(&metadata)
                                } else {
                                   format_size(metadata.len())
                                };
                                
                                writeln!(output, "{}{} {:>2} {:>3} {:>3} {} {} {}", file_type,display_perm(mode), n_link, user, group, size, last_updated, display_name).unwrap();
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
        } else if metadata.permissions().mode() & 0o111 != 0{
            format!("{}*", entry.file_name().to_string_lossy().to_string())
        } else if metadata.is_symlink(){
            format!("{}@", entry.file_name().to_string_lossy().to_string())
        }else {
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

fn get_device_numbers(metadata: &Metadata) -> String {
    let rdev = metadata.rdev();
    let major = (rdev >> 8) as u8;
    let minor = rdev as u8;
    format!("{:>3},  {:>3}", major, minor)
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P", "E"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{:>7}{}", size as u64, UNITS[unit_index])
    } else if size < 10.0 {
        format!("{:>7.1}{}", size, UNITS[unit_index])
    } else {
        format!("{:>7.0}{}", size, UNITS[unit_index])
    }
}

fn get_user_group(metadata: &Metadata) -> (String,String) {
    let uid = metadata.uid();
    let gid = metadata.gid();

    let user = users::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or(uid.to_string());

    let group = users::get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or(gid.to_string());

    (user, group)
}

fn get_modified_time(metadata: &Metadata) -> String {
    match metadata.modified() {
        Ok(modified) => {
            let d: DateTime<Local> = DateTime::from(modified);
            let now = Local::now();

            let diff = now.signed_duration_since(d);
            if diff.num_seconds() < 0 {
                d.format("%b %e %H:%M").to_string()
            } else if diff < Duration::days(180) {
                d.format("%b %e %H:%M").to_string()
            } else {
                d.format("%b %e  %Y").to_string()
            }
        }
        Err(_) => "??? ?? ????".to_string(),
    }
}
