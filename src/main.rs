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

    let mut locations: Vec<Location> = Vec::new();
    let mut start:u64 = 0;
    loop {
        let buf = get_bytes(&config.file_path, start, 28);
        let offset = LE::read_u64(&buf[12..20]);
        log::debug!("offset: {offset}");
        let end = start + offset + 28;
        log::debug!("end: {end}");
        let location = Location::new(start, end);
        start = location.end;
        log::debug!("start: {start}");
        locations.push(location);
    }

    // let mut r = Reader::new(&config.file_path);
    //
    // log::info!(
    //     "Created Reader for {:?}",
    //     config.file_path
    //         .split('/')
    //         .filter(|x| x.contains("tdms"))
    //         .collect::<String>()
    // );
    //
    // println!("{r:?}");
    //
    // for i in 0..9 { 
    //     for _ in 0..122 { print!("-")}; println!();
    //     let li = LeadIn::new(&r.read(28), r.last_pos);
    //     println!("li: {li:?}");
    //     println!();
    //     r.read_meta();
    //     println!("\n========> iter {i} <========");
    // }
    // let _li = LeadIn::new(&r.read(28), r.last_pos);
    //
    // r.get_num_of_objs();
    //
    // loop {
    //     #[allow(unused_assignments)]
    //     let mut num_of_props = 0;
    //     loop {
    //         println!();
    //         num_of_props = r.read_obj();
    //         if num_of_props != 0{
    //                 break;
    //             }
    //     }
    //     for _ in 0..num_of_props {
    //         r.read_prop();
    //     }
    // }
}
