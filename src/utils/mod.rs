use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use std::env::current_exe;
pub mod network;

/// Self replace
pub fn self_replace(source_file: &Path, bak_file: &Path) -> std::io::Result<()> {
    use std::{env, fs};
    if source_file.exists() {
        if cfg!(windows) {
            fs::rename(current_exe().unwrap().as_path(), bak_file)?;
            fs::rename(source_file, current_exe().unwrap().as_path())?;
        }
        if cfg!(unix) {
            fs::rename(current_exe().unwrap().as_path(), bak_file)?;
            fs::copy(source_file, current_exe().unwrap().as_path())?;
        }

        if !env::current_exe().unwrap().as_path().exists() {
            fs::rename(bak_file, current_exe().unwrap().as_path())?;
        } else {
            println!("Replace completed!")
        }
    }
    Ok(())
}

/// Number Counter
/// Exit if the counter is full
#[derive(Debug, Deserialize, Serialize)]
pub struct Counter {
    count: u64,
    total: u64,
    msg: String,
}

impl Counter {
    pub fn new(total: u64) -> Self {
        Self {
            count: 0,
            total,
            msg: "".to_string(),
        }
    }

    pub fn count(&self) -> Self {
        Self {
            count: &self.count + 1,
            total: Default::default(),
            msg: "".to_string(),
        }
    }

    pub fn msg(&self, msg: &str) {
        if &self.total == &self.count {
            println!("[Counter-{}]: {}", &self.count, msg);
            std::process::exit(1);
        }
    }
}
