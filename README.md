
# 0-shell

A minimalist Unix-like shell written in Rust.  
Designed to run core shell commands without relying on external binaries or system shells.

## Overview

0-shell implements basic Unix shell behavior including:

- Navigating directories (`cd`, `pwd`)
- Listing directory contents (`ls -l -a -F`)
- File manipulation (`cat`, `cp`, `rm -r`, `mv`, `mkdir`)
- Printing output (`echo`)
- Exiting the shell (`exit`)

It demonstrates:

- Reading user input in a REPL loop
- Parsing commands and flags
- Handling file system operations with Rust's `std::fs`
- Error handling and graceful shell exit (Ctrl+D)

## Project Structure

```

0-shell/
├── Cargo.toml
└── src/
├── main.rs         # Entry point
├── lib.rs          # Library root
├── error.rs        # Error types
├── utils.rs        # Helper functions
├── behavior/       # Core shell logic
│   ├── mod.rs
│   ├── shell.rs
│   └── parser.rs
└── commands/       # Built-in commands
├── mod.rs
├── cd.rs
├── echo.rs
├── ls.rs
├── pwd.rs
├── exit.rs
├── rm.rs
└── ... (other commands)

````

## Usage

Run the shell:

```bash
cargo run
````

Example session:

```text
$ cd src
$ pwd
/home/user/0-shell/src
$ echo "Hello, 0-shell!"
Hello, 0-shell!
$ ls -l
total 0
-rw-r--r-- 1 user user  0 Sep 19 10:00 main.rs
-rw-r--r-- 1 user user  0 Sep 19 10:00 lib.rs
...
$ exit
```