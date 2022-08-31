use std::fs::File;
use std::fmt;
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
        location
    }
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {{ Start: {}, End: {}, Length: {} }}", self.start, self.end, self.length)
    }
}

pub fn open_file(file_path: String) -> File {
    match File::open(&file_path) {
        Ok(file) => {file},
        Err(e) => panic!("Error reading {file_path}: {e}"),
    }
}

pub fn find_segments(file_path: &String) -> Vec<Location> {
    let mut f = open_file(file_path.to_string());
    let f_len = f.metadata().unwrap().len();
    let mut segments: Vec<Location> = Vec::new();
    let mut buf: [u8;28] = [0u8;28];

    let mut loc = 0;
    while loc < f_len {
        match f.seek(SeekFrom::Start(loc)) {
            Ok(new_loc) => {
                log::debug!("Seeked to {}", new_loc);
                match f.read(&mut buf) {
                Ok(num_bytes_read) => {
                        log::debug!("Read {} bytes to buf", num_bytes_read);
                        let li = leadin::LeadIn::new(&buf, loc);
                        // Validate LeadIn
                        // if li.tag != "TDSm".to_string() || li.tag != "TDSh".to_string() {
                        //     let err_string = format!("LeadIn parse Error. Want: 'TDSm' or 'TDSh' Got: {}", li.tag);
                        //     log::error!("{}\n{}", err_string, li);
                        //     panic!("{}", err_string);
                        // }
                        // Maybe save LeadIn to Vec so you only read LeadIns once?
                        let location = Location::new(li.position, li.position + li.next_segment_offset + 28);
                        log::debug!("{}", location);
                        loc = location.end;
                        log::debug!("New `loc` = {}", loc);
                        segments.push(location);
                        log::debug!("segments.len() = {}", segments.len());
                    },
                Err(e) => {
                        let err_string = format!("File Byte Read Error: {e}");
                        log::error!("{}", err_string);
                        panic!("{}", err_string);
                    }
                }
            },
            Err(e) => {
                let err_string = format!("File Seek Error (loc: {}): {}", loc, e);
                log::error!("{}", err_string);
                panic!("{}", err_string);
            }
        }
    }
    segments
}
