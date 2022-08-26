use std::fs::File;
use std::io::{
    Read,
    Seek,
    SeekFrom,
};

pub mod data;
pub mod leadin;
pub mod object;
pub mod property;
pub mod reader;
pub mod toc;

pub const NO_RAW_DATA: u32 = 0xFFFFFFFF;
pub const DAQMX_FORMAT_CHANGING_SCALER: u32 = 0x69120000;
pub const DAQMX_DIGITAL_LINE_SCALER: u32 = 0x69130000;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config { file_path })
    }
}

#[derive(Debug)]
pub struct Location{
    pub start: u64,
    pub end: u64,
    pub length: u64,
}
impl Location{
    pub fn new(start: u64, end: u64) -> Self {
        let length = end - start;
        let location = Location { start, end, length };
        log::debug!("Location: {:?}", location);
        location
    }
}

pub fn open_file(file_path: String) -> File {
    match File::open(&file_path) {
        Ok(file) => {file},
        Err(e) => panic!("Error reading {file_path}: {e}"),
    }
}

pub fn get_bytes(file_path: &String, loc: u64, num: usize) -> Vec<u8>{
    log::debug!("get_bytes args: loc: {loc}, num: {num}, file_path: {file_path}");
    let mut f = open_file(file_path.to_string());
    let file_metadata = f.metadata().expect("Error getting file metadata");
    let file_len = file_metadata.len();
    if loc >= file_len {return vec!();}
    let mut buffer = vec![0u8; num];
    match f.seek(SeekFrom::Start(loc)) {
        Ok(new_pos) => {
            log::debug!("Seeked from 0 to {}", new_pos);
            match f.read(&mut buffer) {
                Ok(num_read) => {
                    log::debug!("Read {num_read} bytes to buffer");
                    if num_read != num{
                        log::warn!("tdms::get_bytes Read: {num_read} Wanted {num}");
                    }
                },
                Err(e) => {
                    log::error!("tdms::get_bytes Read Error: {e}");
                    panic!("tdms::get_bytes Read Error: {e}");
                }
            }
        },
        Err(e) => {
            log::error!("tdms::get_bytes Seek Error: {e}");
            panic!("tdms::get_bytes Seek Error: {e}");
        }
    }
    buffer
}

pub fn dbg_format_bytes(src: &[u8], pos: &i64) -> String {
    let mut output = String::new();
    let dst = src.chunks(4);
    let pos_str = format!("{pos:06}: ");
    output.push_str(&pos_str);
    let mut indent = false;
    let lines = dst.len() - 1;
    for (line_count, i) in dst.enumerate(){
        if indent {
            for _ in 0..pos_str.len() {
                output.push(' ');
            }
        }
        indent = true;
        output.push_str("[ ");
        for v in i {
            output.push_str(format!("{:02X} ", v).as_str());
        }
        if i.len() < 4 {
            let n = 4 - i.len();
            for _ in 0..n {
                output.push_str("   ");
            }
        }
        if line_count == lines {
            output.push_str("] -> ");
        } else {
            output.push_str("]\n");
        }
    }
    output
}
