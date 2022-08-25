#![allow(unused_imports)]
use std::env;
use std::process;
use tdms::Config;
use tdms::leadin::LeadIn;
use tdms::reader::Reader;
use tdms::reader;
use tdms::data;
use byteorder::{ByteOrder, LittleEndian};

fn main() {
    env_logger::init();
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        log::error!("Proplem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    log::info!("config: {:?}", config);

    let mut r = Reader::new(&config.file_path);

    log::info!(
        "Created Reader for {:?}",
        config.file_path
            .split('/')
            .filter(|x| x.contains("tdms"))
            .collect::<String>()
    );

    println!("{r:?}");

    for i in 0..9 { 
        for _ in 0..122 { print!("-")}; println!();
        let li = LeadIn::new(&r.read(28), r.last_pos);
        println!("li: {li:?}");
        println!();
        r.read_meta();
        println!("\n========> iter {i} <========");
    }
    let li = LeadIn::new(&r.read(28), r.last_pos);
    println!("{:?}", li);
    r.get_num_of_objs();

    for _ in 0..25 {
        #[allow(unused_assignments)]
        let mut num_of_props = 0;
        loop {
            for _ in 0..122 {
                print!("-");
            }
            println!();
            for _ in 0..122 {
                print!("-");
            }
            println!("======================> Object <=======================");
            println!();
            num_of_props = r.read_obj();
            if num_of_props != 0{
                    break;
                }
        }
        println!("====================> Properties <=====================");
        for _ in 0..num_of_props {
            r.read_prop();
        }
    }

    println!("======================> Object <=======================");
    #[allow(unused_variables)]
    let num_of_props = r.read_obj();
    println!("====================> Properties <=====================");
    
    let name_len = r.read_u32();
    reader::dbg_bytes(&r.buffer, &r.last_pos);
    println!("name_len: {name_len}");
    let name = r.read_string(name_len);
    reader::dbg_bytes(&r.buffer, &r.last_pos);
    println!("name: {name}");

    r.read_u32();
    reader::dbg_bytes(&r.buffer, &r.last_pos);
    println!();
    r.read_u32();
    reader::dbg_bytes(&r.buffer, &r.last_pos);
    println!("{}", LittleEndian::read_i32(&r.buffer));
    r.read_u32();
    reader::dbg_bytes(&r.buffer, &r.last_pos);
    println!("{}", LittleEndian::read_u32(&r.buffer));

    println!("\n\n\n{:?}", li);
}
