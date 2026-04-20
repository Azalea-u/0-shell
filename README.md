# 0-shell

A minimal Unix shell built from scratch in Rust.

## Overview

0-shell implements core shell behavior without relying on any shell libraries — from raw input tokenization to process execution and I/O redirection. The architecture mirrors how a real shell works: tokenize, parse, dispatch, execute.

## Features

- **Lexical tokenizer** — handles single quotes, double quotes, escape sequences, and whitespace splitting correctly
- **I/O redirection** — supports `>` (overwrite), `>>` (append), and `<` (input) operators
- **Built-in commands** — `ls`, `cd`, `pwd`, `echo`
  - `ls` supports `-a` (all), `-l` (long format), `-F` (classify) flags
  - `cd` tracks logical and physical working directories, supports `cd -`
- **Prompt** — displays current working directory, updates on navigation
- **Multi-line input** — detects unclosed quotes and continues input on next line
- **Signal handling** — structured process and error management via `signal-hook`

## Architecture

```
src/
├── main.rs
├── lib.rs
├── behavior/
│   ├── shell.rs       # Shell state: cwd, prompt, dir tracking
│   ├── tokenizer.rs   # Lexer + redirect parser
│   └── parser.rs      # Token → Command struct
├── core/
│   └── redirect.rs    # I/O stream resolution (>, >>, <)
└── commands/
    ├── ls.rs
    ├── cd.rs
    ├── pwd.rs
    └── echo.rs
```

## Getting Started

```bash
git clone https://github.com/D0ulo5/0-shell
cd 0-shell
cargo run
```

## Built With

- [Rust](https://www.rust-lang.org/)
- [`signal-hook`](https://docs.rs/signal-hook) — signal handling
- [`chrono`](https://docs.rs/chrono) — timestamps in `ls -l`
- [`users`](https://docs.rs/users) — user/group info in `ls -l`
