use std::io;
//Shell basic struct
use std::path;
use std::env;
pub struct Shell {
    pub l_cwd: path::PathBuf,
    pub p_cwd: path::PathBuf,
    pub old_cwd: Option<path::PathBuf>,
    pub gate_opened: bool
}

impl Shell {
    pub fn new() -> Self {
        let logical = env::current_dir().unwrap_or_else(|_| path::PathBuf::from("/"));
        let physicl = env::current_dir().unwrap_or_else(|_| logical.clone());
        return Self {
            l_cwd: logical,
            p_cwd: physicl,
            old_cwd: None,
            gate_opened: false
        }
    }
    pub fn promt(&self) -> String {
        if self.gate_opened{
            format!("> ")
        } else {
            format!("{}$ ", self.l_cwd.display())
        }
    }

     pub fn refresh(&mut self) {
        if let Ok(current) = env::current_dir() {
            self.l_cwd = current.clone();
            self.p_cwd = current.canonicalize().unwrap_or(current);
        }
    }

    pub fn update_dir(&mut self, new_dir: path::PathBuf) -> io::Result<()> {
        self.old_cwd = Some(self.l_cwd.clone());
        
        env::set_current_dir(&new_dir)?;
        
        self.l_cwd = new_dir.clone();
        self.p_cwd = new_dir.canonicalize().unwrap_or(new_dir.clone());

        Ok(())
    }
}
