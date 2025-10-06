//! Shell behavior
use std::path::PathBuf;
use std::env;

pub struct Shell {
    pub cwd: PathBuf,
    pub old_cwd: Option<PathBuf>,
}

impl Shell {
    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        Self {
            cwd,
            old_cwd: None,
        }
    }

    pub fn prompt(&self) -> String {
        format!("{}$ ", self.cwd.display())
    }

    pub fn refresh(&mut self) {
        if let Ok(current) = env::current_dir() {
            self.cwd = current;
        }
    }

    pub fn update_dir(&mut self, new_dir: PathBuf) {
        self.old_cwd = Some(self.cwd.clone());
        self.cwd = new_dir.clone();
        let _ = env::set_current_dir(&new_dir);
    }
}
