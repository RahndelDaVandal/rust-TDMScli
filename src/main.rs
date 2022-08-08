#![allow(dead_code, unused_imports, unused_variables)]
use byteorder::{ByteOrder, LittleEndian};
use reader::Reader;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use utils::{default_path, print_type_of, type_of};
use dtype::{Dtype, get_dtype};
use meta_data::{MetaData, Object, Property, Value};

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
        // build path from args
    } else {
        file_path = default_path();
    }
    dbg!(&file_path, &file_path.exists());

    let mut r = Reader::new(file_path);
    let l1 = r.read_lead_in();
    dbg!(l1);

    let mut meta = MetaData::new();
    let mut obj = Object::new();
    let mut prop = Property::new();

    meta.set_num_of_objs(&r.read_next(4));

    obj.set_len_of_path(&r.read_next(4));
    obj.set_path(&r.read_next(*obj.len_of_path()));
    obj.set_raw_index(&r.read_next(4));
    obj.set_num_of_properties(&r.read_next(4));
    
    // len of Name
    prop.set_len_of_name(&r.read_next(4));
    // Name
    prop.set_name(&r.read_next(*prop.len_of_name()));
    // dtype
    prop.set_dtype(&r.read_next(4));
    // len of Value if string
    prop.set_len_value(&r.read_next(4));
    // value
    prop.set_value(&r.read_next(*prop.len_of_name()));

    obj.add_property(prop);
    meta.add_object(obj);
    dbg!(&meta);
    // dbg!(&obj);


    // // Number of Objects
    // let mut buf = r.read_next(4);
    // let num_objs = LittleEndian::read_u32(&buf);
    // dbg!(num_objs);
    //
    // // Length of Object Path
    // buf = r.read_next(4);
    // let len_obj_name = LittleEndian::read_u32(&buf);
    // dbg!(len_obj_name);
    //
    // // Object Path
    // buf = r.read_next(len_obj_name as usize);
    // let obj_path = String::from_utf8(buf).unwrap();
    // dbg!(obj_path);
    //
    // // Raw DataIndex Info
    // buf = r.read_next(4);
    // println!("{:x} {:x} {:x} {:x}", buf[0], buf[1], buf[2], buf[3]);
    // let raw_idx = LittleEndian::read_u32(&buf);
    // dbg!(raw_idx);
    //
    // // Number of Properties
    // buf = r.read_next(4);
    // let num_props = LittleEndian::read_u32(&buf);
    // dbg!(num_props);
    //
    // // Length of Property Name
    // buf = r.read_next(4);
    // let len_prop = LittleEndian::read_u32(&buf);
    // dbg!(len_prop);
    //
    // // Property Name
    // buf = r.read_next(len_prop as usize);
    // let prop_name = String::from_utf8_lossy(&buf);
    // dbg!(prop_name);
    //
    // // Property Dtype
    // buf = r.read_next(4);
    // let prop_dtype = LittleEndian::read_u32(&buf);
    // dbg!(get_dtype(prop_dtype));
    //
    // // Len of Property Value - If string [20 00 00 00] tdsTypeString
    // buf = r.read_next(4);
    // let prop_val_len = LittleEndian::read_u32(&buf);
    // dbg!(prop_val_len);
    //
    // // Property Value
    // buf = r.read_next(prop_val_len as usize);
    // let prop_val = String::from_utf8_lossy(&buf);
    // dbg!(prop_val);
    //
    // // Length of Object Name
    // buf = r.read_next(4);
    // let len_obj_name = LittleEndian::read_u32(&buf);
    // dbg!(len_obj_name);
    //
    // // Object Name
    // buf = r.read_next(len_obj_name as usize);
    // let obj_path = String::from_utf8(buf).unwrap();
    // dbg!(obj_path);
    //
    // // Raw DataIndex Info
    // buf = r.read_next(4);
    // println!("{:x} {:x} {:x} {:x}", buf[0], buf[1], buf[2], buf[3]);
    // let len_index = LittleEndian::read_u32(&buf);
    // dbg!(len_index);
    //
    // // Number of Properties
    // buf = r.read_next(4);
    // let num_props = LittleEndian::read_u32(&buf);
    // dbg!(num_props);
    //
    // // Length of Object Name
    // buf = r.read_next(4);
    // let len_obj_name = LittleEndian::read_u32(&buf);
    // dbg!(len_obj_name);
    //
    // // Object Name
    // buf = r.read_next(len_obj_name as usize);
    // let obj_path = String::from_utf8(buf).unwrap();
    // dbg!(obj_path);
    //
    // // Raw DataIndex Info
    // buf = r.read_next(4);
    // println!("{:x} {:x} {:x} {:x}", buf[0], buf[1], buf[2], buf[3]);
    // let len_index = LittleEndian::read_u32(&buf);
    // dbg!(len_index);
    //
    // // Raw Data Dtype
    // buf = r.read_next(4);
    // let raw_dtype = LittleEndian::read_u32(&buf);
    // dbg!(get_dtype(raw_dtype));
    //
    // // Raw Data Dimension
    // buf = r.read_next(4);
    // println!("{:x} {:x} {:x} {:x}", buf[0], buf[1], buf[2], buf[3]);
    // let data_dim = LittleEndian::read_u32(&buf);
    // dbg!(data_dim);
    //
    // // Number of Values
    // buf = r.read_next(8);
    // let num_vals = LittleEndian::read_u64(&buf);
    // dbg!(num_vals);
    //
    // dbg!(r.location);
}
