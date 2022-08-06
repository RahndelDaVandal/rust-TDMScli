#![allow(dead_code, unused_imports, unused_variables)]
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use reader::Reader;

mod lead_in;
mod reader;
mod toc;

fn main() {
    let toc_hash = HashMap::from([
        ("TocMetaData", 1 << 1),
        ("TocRawData", 1 << 3),
        ("TocDAQmxRawData", 1 << 7),
        ("TocInterleavedData", 1 << 5),
        ("TocBigEndian", 1 << 6),
        ("TocNewObjList", 1 << 2),
    ]);

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

    let mut r = Reader::new(file_path);
    let l1 = r.read_lead_in();
    dbg!(l1);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
