use std::env;
use std::io::BufReader;
use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::Read;

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

    let mut file = File::open(file_path).expect("Error reading file");
    // let mut handle = file.take(4);
    // let mut buffer = [0; 4];

    // handle.read(&mut buffer).expect("err reading handle");

    // let s = String::from_utf8_lossy(&mut buffer);
    // println!("{}", s);
    let buffer = read_4bytes(&mut file);
    println!("{:?}",String::from_utf8_lossy(&buffer));
    let buffer = read_4bytes(&mut file);
    println!("{:?}", buffer);
    let buffer = read_4bytes(&mut file);
    println!("{:?}", buffer);
    let buffer = read_8bytes(&mut file);
    println!("{:?}", buffer);
    let buffer = read_8bytes(&mut file);
    println!("{:?}", buffer);
}

fn read_4bytes(file: &mut File) -> [u8; 4]{
    let mut buffer = [0; 4];
    let mut handle = file.take(4);
    handle.read(&mut buffer).expect("error reading bytes");
    return buffer
}

fn read_8bytes(file: &mut File) -> [u8; 8]{
    let mut buffer = [0; 8];
    let mut handle = file.take(8);
    handle.read(&mut buffer).expect("error reading bytes");
    return buffer
}
