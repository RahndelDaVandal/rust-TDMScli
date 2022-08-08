#![allow(dead_code, unused_imports, unused_variables)]
use std::fs::File;
use std::io::BufReader;
use byteorder::{ByteOrder, LittleEndian};
use reader::Reader;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use utils::{default_path, print_type_of, type_of};
use dtype::{Dtype, get_dtype};
use meta_data::{MetaData, Object, Property};

mod dtype;
mod lead_in;
mod reader;
mod toc;
mod utils;
mod meta_data;

fn main() {
    let cmd_args: Vec<String> = env::args().collect();
    dbg!(&cmd_args);

    let mut file_path = PathBuf::new();

    if cmd_args.len() > 1 {
        // TODO - build path from args
    } else {
        file_path = default_path();
    }
    dbg!(&file_path, &file_path.exists());

    let mut r = Reader::new(file_path);
    let l1 = r.read_lead_in();
    dbg!(l1);

    let mut meta = MetaData::new();
    let buf = r.read_next(4);
    meta.set_num_of_objs(&buf);
    
    let obj = get_obj(&mut r);
    
    // // Get Property Logic Test
    // // Read Property Name
    // let buf = r.read_next(4);
    // let len_property_name = LittleEndian::read_u32(&buf);
    // let property_name = r.read_next(len_property_name);
    // // Read Property Value
    // let buf = r.read_next(4);
    // let dtype = LittleEndian::read_u32(&buf);
    // let prop = match dtype{
    //     0x20 => {
    //         let len_dtype_string = r.read_next(4);
    //         let v = r.read_next(LittleEndian::read_u32(&len_dtype_string));
    //         Property{
    //             name: String::from_utf8_lossy(&property_name).to_string(),
    //             value: get_dtype(dtype, &v),
    //         }
    //     },
    //     _ => {
    //         let v = r.read_next(4);
    //         Property{
    //             name: String::from_utf8_lossy(&property_name).to_string(),
    //             value: get_dtype(dtype, &v),
    //         }
    //     },
    // };
    // 
    // obj.add_property(prop);
    meta.add_object(obj);
    dbg!(&meta);
}

fn get_obj(reader: &mut Reader) -> Object{
    // len_of_path
    let mut buf = reader.read_next(4);
    let len_of_path = LittleEndian::read_u32(&buf);
    // path
    buf = reader.read_next(len_of_path);
    let path = String::from_utf8_lossy(&buf).to_string();
    // raw_index
    buf = reader.read_next(4);
    let raw_index = LittleEndian::read_u32(&buf);
    // num_props
    buf = reader.read_next(4);
    let num_of_properties = LittleEndian::read_u32(&buf);

    Object{
        path,
        raw_index,
        num_of_properties,
        properties: Vec::new()
    }
}

fn get_prop(){
}
