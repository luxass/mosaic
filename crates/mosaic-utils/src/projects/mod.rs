use std::{env, fs};
use std::path::PathBuf;

pub async fn is_mosaic_project() -> bool {
    let cwd: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(_) => return false,
    };


    match fs::metadata(cwd.join(".github/mosaic.toml")) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}