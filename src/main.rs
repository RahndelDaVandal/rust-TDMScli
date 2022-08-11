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
use toc::ToC;

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
    dbg!(&l1);

    println!("Before MetaData: {}", &r.location);

    let mut meta = MetaData::new();
    let mut buf = r.read_next(4);
    meta.set_num_of_objs(&buf);
    
    let mut obj = get_obj(&mut r);
    let mut prop = get_prop(&mut r);
    obj.add_property(prop);
    meta.add_object(obj);

    println!("After first Object: {}", &r.location);

    obj = get_obj(&mut r);
    meta.add_object(obj);

    println!("After second Object: {}", &r.location);

    obj = get_obj(&mut r);
    println!("After 3rd Obj before props: {}", &r.location);

    meta.add_object(obj);
    dbg!(&meta);
    dbg!(&r.location);

    // let toc_hash = HashMap::from([
    //     ("TocMetaData", 1 << 1),
    //     ("TocRawData", 1 << 3),
    //     ("TocDAQmxRawData", 1 << 7),
    //     ("TocInterleavedData", 1 << 5),
    //     ("TocBigEndian", 1 << 6),
    //     ("TocNewObjList", 1 << 2),
    // ]);
    let toc_hash = HashMap::from([
        (ToC::TocMetaData, 1 << 1),
        (ToC::TocRawData, 1 << 3),
        (ToC::TocDAQmxRawData, 1 << 7),
        (ToC::TocInterleavedData, 1 << 5),
        (ToC::TocBigEndian, 1 << 6),
        (ToC::TocNewObjList, 1 << 2),
    ]);

    let m = l1.toc;

    let mut flags: Vec<ToC> = Vec::new();
    for (k, v) in toc_hash{
        println!("{k} ({v:08b} & {:08b} != 0) = {}", &m, (&m & v != 0));
        if &m &v != 0{
            flags.push(k);
        }
    }
    dbg!(&flags);
    dbg!(&m & ToC::TocNewObjList != 0)

}

fn get_obj(r: &mut Reader) -> Object{
    let mut buf = r.read_next(4);
    let len_of_path = LittleEndian::read_u32(&buf);
    buf = r.read_next(len_of_path);
    let path = String::from_utf8_lossy(&buf).to_string();
    buf = r.read_next(4);
    let raw_index = LittleEndian::read_u32(&buf);
    buf = r.read_next(4);
    let num_of_properties = LittleEndian::read_u32(&buf);

    Object{
        path,
        raw_index,
        num_of_properties,
        properties: Vec::new()
    }
}

fn get_prop(r: &mut Reader) -> Property{
    let buf = r.read_next(4);
    let len_property_name = LittleEndian::read_u32(&buf);
    let property_name = r.read_next(len_property_name);
    let buf = r.read_next(4);
    let dtype = LittleEndian::read_u32(&buf);

    match dtype{
        0x20 => {
            let len_dtype_string = r.read_next(4);
            let v = r.read_next(LittleEndian::read_u32(&len_dtype_string));
            Property{
                name: String::from_utf8_lossy(&property_name).to_string(),
                value: get_dtype(dtype, &v),
            }
        },
        _ => {
            let v = r.read_next(4);
            Property{
                name: String::from_utf8_lossy(&property_name).to_string(),
                value: get_dtype(dtype, &v),
            }
        },
    }
}
