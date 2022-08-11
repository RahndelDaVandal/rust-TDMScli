#![allow(dead_code, unused_imports, unused_variables)]
use byteorder::{ByteOrder, LittleEndian};
use dtype::{get_val_by_dtype, get_dtype_as_string, Dtype};
use meta_data::{MetaData, Object, Property};
use reader::Reader;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use toc::{get_flags, Flag};
use utils::{default_path, print_type_of, type_of};

mod dtype;
mod lead_in;
mod meta_data;
mod reader;
mod toc;
mod utils;

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
    dbg!(&l1, &r.location);

    // Number of Objects
    let mut buf = r.read_next(4);
    let num_of_objs = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("num_of_objs: {num_of_objs}");

    // Length of first object path
    buf = r.read_next(4);
    let len_of_o1_path = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("len_of_o1_path: {len_of_o1_path}");

    // First object path
    buf = r.read_next(len_of_o1_path);
    let o1_path = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_path: \"{o1_path}\"");

    // Raw Data index
    buf = r.read_next(4);
    let o1_raw_idx = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_raw_idx: {o1_raw_idx}");

    // Object1 # of Props
    buf = r.read_next(4);
    let o1_num_of_props = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_num_of_props: {o1_num_of_props}");

    // Object1 Prop1 name length
    buf = r.read_next(4);
    let o1_p1_len_name = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p1_len_name: {o1_p1_len_name}");

    // Object1 Prop1 name
    buf = r.read_next(o1_p1_len_name);
    let o1_p1_name = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p1_name: {o1_p1_name}");

    // Object1 Prop1 Dtype
    buf = r.read_next(4);
    let o1_p1_dtype = get_dtype_as_string(LittleEndian::read_u32(&buf));
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p1_dtype: {o1_p1_dtype}");

    // Object1 Prop1 Value length
    buf = r.read_next(4);
    let o1_p1_val_len = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p1_val_len: {o1_p1_val_len}");

    // Object2 Prop1 Value
    buf = r.read_next(o1_p1_val_len);
    let o2_p1_val = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o2_p1_val: {o2_p1_val}");

    // Object2 Path Length
    buf = r.read_next(4);
    let len_of_o2_path = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("len_of_o2_path: {len_of_o2_path}");

    // Object2 path
    buf = r.read_next(len_of_o2_path);
    let o2_path = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o2_path: \"{o2_path}\"");

    // Object2 Raw Data index
    buf = r.read_next(4);
    let o2_raw_idx = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o2_raw_idx: {o2_raw_idx}");

    // Object2 # of Props
    buf = r.read_next(4);
    let o2_num_of_props = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o2_num_of_props: {o2_num_of_props}");

    // Object3 path length
    buf = r.read_next(4);
    let len_of_o3_path = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("len_of_o3_path: {len_of_o3_path}");

    // Object3 path
    buf = r.read_next(len_of_o3_path);
    let o3_path = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o3_path: \"{o3_path}\"");

    // Object3 Raw Data index
    buf = r.read_next(4);
    let o3_raw_idx = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o3_raw_idx: {o3_raw_idx}");

    // Object3 # of Props
    buf = r.read_next(4);
    let o3_num_of_props = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o3_num_of_props: {o3_num_of_props}");

    // Object3 Prop1 name length
    buf = r.read_next(4);
    let o3_p1_len_name = LittleEndian::read_u32(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o3_p1_len_name: {o3_p1_len_name}");

    // Object3 Prop1 name
    buf = r.read_next(o3_p1_len_name);
    let o3_p1_name = String::from_utf8_lossy(&buf);
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p3_name: {o3_p1_name}");

    // Object3 Prop1 Dtype
    buf = r.read_next(4);
    let o3_p1_dtype = get_dtype_as_string(LittleEndian::read_u32(&buf));
    print!("{:04} : {:02X?} -> ", r.location, buf);
    println!("o1_p3_dtype: {o3_p1_dtype}");


}

fn get_obj(r: &mut Reader) -> Object {
    let mut buf = r.read_next(4);
    let len_of_path = LittleEndian::read_u32(&buf);
    buf = r.read_next(len_of_path);
    let path = String::from_utf8_lossy(&buf).to_string();
    buf = r.read_next(4);
    let raw_index = LittleEndian::read_u32(&buf);
    buf = r.read_next(4);
    let num_of_properties = LittleEndian::read_u32(&buf);

    Object {
        path,
        raw_index,
        num_of_properties,
        properties: Vec::new(),
    }
}

fn get_prop(r: &mut Reader) -> Property {
    let buf = r.read_next(4);
    let len_property_name = LittleEndian::read_u32(&buf);
    let property_name = r.read_next(len_property_name);
    let buf = r.read_next(4);
    let dtype = LittleEndian::read_u32(&buf);

    match dtype {
        0x20 => {
            let len_dtype_string = r.read_next(4);
            let v = r.read_next(LittleEndian::read_u32(&len_dtype_string));
            Property {
                name: String::from_utf8_lossy(&property_name).to_string(),
                value: get_val_by_dtype(dtype, &v),
            }
        }
        _ => {
            let v = r.read_next(4);
            Property {
                name: String::from_utf8_lossy(&property_name).to_string(),
                value: get_val_by_dtype(dtype, &v),
            }
        }
    }
}
