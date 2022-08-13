#![allow(dead_code, unused_imports, unused_variables)]
use byteorder::{ByteOrder, LittleEndian};
use dtype::{get_dtype_as_string, get_val_by_dtype, Dtype};
use reader::Reader;
use segment::{LeadIn, Property, Segment};
use std::collections::HashMap;
use std::{env, fs::File, io::BufReader, path::PathBuf};
use toc::{get_flags, Flag};
use utils::{default_path, print_type_of, type_of};
use crate::reader::dbg_format_bytes;

mod dtype;
mod meta_data;
mod reader;
mod segment;
mod toc;
mod utils;

const NO_RAW_DATA: u32 = 0xFFFFFFFF;
const DAQMX_FORMAT_CHANGING_SCALER: u32 = 0x69120000;
const DAQMX_DIGITAL_LINE_SCALER: u32 = 0x69130000;

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    // dbg!(&cmd_args);
    let mut file_path = PathBuf::new();
    if cmd_args.len() > 1 {
        // TODO - build path from args
    } else {
        file_path = default_path();
    }
    // dbg!(&file_path, &file_path.exists());

    let mut r = Reader::new(file_path);

    let mut segments:Vec<Segment> = Vec::new();

    for _ in 0..10 {
        segments.push(r.read_segment());
    }

    println!("\n");
    for s in segments{
        if s.lead_in.toc_flags.contains(&Flag::NewObjList){
            for o in s.objects{
                println!("{}", o.path);
                for p in o.properties {
                    println!("\t{} = {:?}", p.name, p.value);
                }
            }
            for _ in 0..122 {print!("-");} println!("");
        }
    }
}
