use std::env;
use std::path::PathBuf;

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn default_path() -> PathBuf {
    let mut path = PathBuf::new();
    match env::current_dir() {
        Ok(mut cwd) => {
            cwd.push("data");
            cwd.push("2020-09-17T22-45-47_.tdms");
            path = cwd
        }
        Err(e) => {
            println!("Error creating file_path: {}", e)
        }
    }

    path
}
