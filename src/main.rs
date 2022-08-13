#![allow(dead_code, unused_imports, unused_variables)]
use byteorder::{ByteOrder, LittleEndian};
use dtype::{get_val_by_dtype, get_dtype_as_string, Dtype};
use reader::Reader;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use toc::{get_flags, Flag};
use utils::{default_path, print_type_of, type_of};
use segment::{Segment, LeadIn, Property, read_lead_in, dbg_lead_in, dbg_format_bytes};

mod dtype;
mod meta_data;
mod reader;
mod toc;
mod utils;
mod segment;

const NO_RAW_DATA:u32 = 0xFFFFFFFF;
const DAQMX_FORMAT_CHANGING_SCALER:u32 = 0x69120000;
const DAQMX_DIGITAL_LINE_SCALER:u32 = 0x69130000;

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
    // r.move_to(99958);

    let mut _li = dbg_lead_in(&mut r);

    let num_of_objs = r.get_num_objs();

    for _ in 0..num_of_objs{
        let path = r.get_path();
        let raw_data_index = r.get_raw_index();
        if raw_data_index == NO_RAW_DATA{
            let num_of_props = r.get_num_of_props();
            if num_of_props > 0 {
                for _ in 0..num_of_props{
                    let prop = r.get_prop();
                }
            }
        } else {
            let data_dtype = r.get_data_dtype();
            let data_dimension = r.get_data_dimension();
            let num_of_data_values = r.get_num_of_data_values();
            let num_of_props = r.get_num_of_props();
            if num_of_props > 0 {
                for _ in 1..num_of_props{
                    let prop = r.get_prop();
                }
            }
        }
    };

    // let path = r.get_path();
    // let raw_data_index = r.get_raw_index();
    // let num_of_props = r.get_num_of_props();
    // let prop = r.get_prop();

    // let path = r.get_path();
    // let raw_data_index = r.get_raw_index();
    // let num_of_props = r.get_num_of_props();

    // let path = r.get_path();
    // let raw_data_index = r.get_raw_index();
    // let data_dtype = r.get_data_dtype();
    // let data_dimension = r.get_data_dimension();
    // let num_of_data_values = r.get_num_of_data_values();
    // let num_of_props = r.get_num_of_props();

    // let mut i = 0;
    // loop {
    //     let chan = read_channel(&mut r, false);
    //     println!("{} -> \"{}\"", i, chan.path);
    //     i += 1;
    //     if chan.num_of_props != 0{break;}
    // }
}

struct Channel{
    path_len: u32,
    path: String,
    len_index_info: u32,
    data_type: String, // TODO - possibly seperate dtype enum w/ and w/o actual value
    dimension: u32,
    num_of_raw_values: u64,
    num_of_props: u32,
}

fn read_channel(r: &mut Reader, verbose: bool) -> Channel{
    // Channel Path Length
    let b_path_len = r.read_next(4);
    let path_len = LittleEndian::read_u32(&b_path_len);
    if path_len > 1000 as u32{
        panic!("WARNING! - Channel Path Length is greater than 1000! ({path_len})");
    }
    // Channel Path
    let b_path = r.read_next(path_len);
    let path = String::from_utf8_lossy(&b_path).to_string();
    // Channel Index Info Length
    let b_len_index_info = r.read_next(4);
    let len_index_info = LittleEndian::read_u32(&b_len_index_info);
    // Channel Data Type as String
    let b_dtype = r.read_next(4);
    let dtype_u32 = LittleEndian::read_u32(&b_dtype);
    let data_type = get_dtype_as_string(dtype_u32).to_string();
    // Data Dimension
    let b_dimension = r.read_next(4);
    let dimension = LittleEndian::read_u32(&b_dimension);
    // NUmber of Raw Data Values
    let b_num_of_raw_values = r.read_next(8);
    let num_of_raw_values = LittleEndian::read_u64(&b_num_of_raw_values);
    // Number of Properties
    let b_num_of_props = r.read_next(4);
    let num_of_props = LittleEndian::read_u32(&b_num_of_props);

    if verbose{
        // Channel Path Length
        print!("{}", dbg_format_bytes(&b_path_len, &r.location));
        println!("PATH_LEN: {path_len}\n");
        // Channel Path
        print!("{}", dbg_format_bytes(&b_path, &r.location));
        println!("PATH: \"{path}\"\n");
        // Channel Index Info Length
        print!("{}", dbg_format_bytes(&b_len_index_info, &r.location));
        println!("LEN_INDEX_INFO: {len_index_info}\n");
        // Channel Data Type as String
        print!("{}", dbg_format_bytes(&b_dtype, &r.location));
        println!("DATA_TYPE: {data_type}\n");
        // Data Dimension
        print!("{}", dbg_format_bytes(&b_dimension, &r.location));
        println!("DIMENSION: {dimension}\n");
        // NUmber of Raw Data Values
        print!("{}", dbg_format_bytes(&b_num_of_raw_values, &r.location));
        println!("#_OF_RAW_VALUES: {num_of_raw_values}\n");
        // Number of Properties
        print!("{}", dbg_format_bytes(&b_num_of_props, &r.location));
        println!("#_OF_PROPERTIES: {num_of_props}\n");
    }

    Channel{
        path_len,
        path,
        len_index_info,
        data_type,
        dimension,
        num_of_raw_values,
        num_of_props
    }
}
