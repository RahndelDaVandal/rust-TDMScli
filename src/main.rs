#![allow(unused_imports)]
use std::io::{Read, BufReader, Seek};
use std::env;
use std::process;
use tdms::Config;
use tdms::leadin::LeadIn;
use tdms::reader::Reader;
use tdms::reader;
use tdms::data;
use tdms::open_file;
use tdms::get_bytes;
use tdms::Location;
use tdms::find_segments;
use byteorder::ByteOrder;
use byteorder::LittleEndian as LE;

fn main() {
    env_logger::init();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        log::error!("Proplem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    log::info!("config: {:?}", config);

    let segments = find_segments(&config.file_path);
    println!("num of segments = {}", segments.len());
}
