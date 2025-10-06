//! Shell behavior
use std::path::PathBuf;
use std::env;

pub struct Shell {
    pub cwd: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        Self { cwd }
    }

    pub fn prompt(&self) -> String {
        format!("{}$ ", self.cwd.display())
    }

    pub fn refresh(&mut self) {
        if let Ok(current) = env::current_dir() {
            self.cwd = current;
        }
    }
}
