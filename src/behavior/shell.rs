//Shell basic struct
use std::path;
use std::env;
pub struct Shell {
    pub l_cwd: path::PathBuf,
    pub p_cwd: path::PathBuf,
    pub old_cwd: Option<path::PathBuf>,
    pub gate_open: bool
}

impl Shell {
    pub fn new() -> Self {
        let logical = env::current_dir().unwrap_or_else(|_| path::PathBuf::from("/"));
        let physicl = env::current_dir().unwrap_or_else(|_| logical.clone());
        return Self {
            l_cwd: logical,
            p_cwd: physicl,
            old_cwd: None,
            gate_open: false
        }
    }
    pub fn promt(&self) -> String {
        if self.gate_open{
            format!("> ")
        } else {
            format!("{}: $ ", self.l_cwd.display())
        }
    }

     pub fn refresh(&mut self) {
        if let Ok(current) = env::current_dir() {
            self.l_cwd = current.clone();
            self.p_cwd = current.canonicalize().unwrap_or(current);
        }
    }

    pub fn update_dir(&mut self, new_dir: path::PathBuf) {
        self.old_cwd = Some(self.l_cwd.clone());
        self.l_cwd = new_dir.clone();
        self.p_cwd = new_dir.canonicalize().unwrap_or(new_dir.clone());
        let _ = env::set_current_dir(&new_dir);
    }
}
