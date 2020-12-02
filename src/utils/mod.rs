use std::{env, fs};
use std::fs::File;
use std::path::Path;

use structopt::clap::Values;

pub mod counter;
pub mod network;

pub fn self_replace(source_file: &Path, bak_file: &Path) -> std::io::Result<()> {
    if source_file.exists() {
        fs::rename(env::current_exe().unwrap().as_path(), bak_file)?;
        fs::rename(source_file, env::current_exe().unwrap().as_path())?
    }
    Ok(())
}

pub fn get_version() {
    todo!()
}