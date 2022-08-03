use std::env;
use std::path::{Path,PathBuf};
use std::fs::File;

fn main() {
    let mut file_path = PathBuf::new();
    match env::current_dir() {
        Ok(mut cwd) => {
            cwd.push("data");
            cwd.push("2020-09-17T22-45-47_.tdms");
            dbg!(&cwd);
            file_path = cwd
        }
        Err(e) => {
            println!("Error creating file_path: {}", e)
        }
    }
    dbg!(&file_path, &file_path.exists());
}

