use byteorder::{ByteOrder, LittleEndian};
use std::fmt;

use crate::reader::Reader;
use crate::toc::{get_flags, Flag};
use crate::dtype::Dtype;

#[derive(Debug)]
pub struct Segment {
    pub lead_in: LeadIn,
    pub num_of_objects: u32,
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    pub dtype: String,
    pub value: Dtype,
}

#[derive(Debug)]
pub struct LeadIn {
    pub position: i32,
    pub tag: String,
    pub toc_flags: Vec<Flag>,
    pub version: u32,
    pub next_segment_offset: u64,
    pub raw_data_offset: u64,
}

pub fn read_lead_in(r: &mut Reader) -> LeadIn {
    let position = r.location;
    let b_tag = r.read_next(4);
    let b_toc_mask = r.read_next(4);
    let b_version = r.read_next(4);
    let b_next_segment_offset = r.read_next(8);
    let b_raw_data_offset = r.read_next(8);

    let tag = String::from_utf8_lossy(&b_tag).to_string();
    if tag != "TDSh".to_string() || tag != "TDSm".to_string() {
        eprintln!("Error: unexpected TDMS file tag {tag}");
    }

    let toc_mask = LittleEndian::read_i32(&b_toc_mask);
    let toc_flags = get_flags(&toc_mask);

    // TODO - Logic based on toc_flags

    let version = LittleEndian::read_u32(&b_version);
    if version != 4712 || version != 4713 {
        eprintln!("Error: unexpected TDMS file version {version}");
    }

    let lead_size = 7 * 4;
    let next_segment_offset = LittleEndian::read_u64(&b_next_segment_offset);
    let raw_data_offset = LittleEndian::read_u64(&b_raw_data_offset);
    let data_position = position + lead_size + raw_data_offset as i32;

    let segment_incomplete = next_segment_offset == 0xFFFFFFFFFFFFFFFF;
    if segment_incomplete {
        println!("{} SEGMENT INCOMPLETE", file!())
    }
    // TODO - Incomplete segment Logic

    LeadIn {
        position,
        tag,
        toc_flags,
        version,
        next_segment_offset,
        raw_data_offset,
    }
}

pub fn dbg_lead_in(r: &mut Reader) -> LeadIn{
    // Print Divider
    for i in 1..123 {
        print!("-");
    }
    println!("");

    // LeadIn position
    let position = r.location;
    println!("LEAD IN AT POSITION {position}\n");

    // File Tag
    let mut buf = r.read_next(4);
    let tag = String::from_utf8_lossy(&buf).to_string();
    print!("{}", dbg_format_bytes(&buf, &r.location));
    println!("TAG: {tag}\n");

    // ToC Flags
    buf = r.read_next(4);
    let toc_mask = LittleEndian::read_i32(&buf);
    let toc_flags = get_flags(&toc_mask);
    // let mut output_str = dbg_format_bytes(&buf, &r.location);
    // output_str.push_str("ToC_FLAGS: [ ");
    print!("{}", dbg_format_bytes(&buf, &r.location));
    print!("ToC_FLAGS: [ ");
    for i in 0..toc_flags.len(){
        print!("{} ", toc_flags[i]);
    }
    println!("]\n");

    // let prefix_len = output_str.len();
    // let mut indent = false;
    //
    // for i in 0..toc_flags.len(){
    //     if indent{
    //         for _ in 0..prefix_len{
    //             output_str.push_str(" ");
    //         }
    //     }
    //     indent = true;
    //
    //     if i == toc_flags.len() - 1{
    //     output_str.push_str(format!("{} ]\n", toc_flags[i]).as_str());
    //     } else {
    //     output_str.push_str(format!("{}\n", toc_flags[i]).as_str());
    //     }
    // }
    // println!("{}", output_str);
    

    // Version
    buf = r.read_next(4);
    let version = LittleEndian::read_u32(&buf);
    print!("{}", dbg_format_bytes(&buf, &r.location));
    println!("VERSION: {version}\n");

    // Next Segment Offset
    buf = r.read_next(8);
    let next_segment_offset = LittleEndian::read_u64(&buf);
    print!("{}", dbg_format_bytes(&buf, &r.location));
    println!("NEXT_SEGMENT_OFFSET: {next_segment_offset}\n");

    // Raw Data Offset
    buf = r.read_next(8);
    let raw_data_offset = LittleEndian::read_u64(&buf);
    print!("{}", dbg_format_bytes(&buf, &r.location));
    println!("RAW_DATA_OFFSET: {raw_data_offset}");

    // Print Divider
    for i in 1..123 {
        print!("-");
    }
    println!("");

    return LeadIn{
        position,
        tag,
        toc_flags,
        version,
        next_segment_offset,
        raw_data_offset,
    };
}

pub fn dbg_format_bytes(src: &Vec<u8>, pos: &i32) -> String {
    let mut output = String::new();
    let dst = src.chunks(4);
    let pos_str = format!("{pos:06}: ");
    output.push_str(&pos_str);
    let mut indent = false;
    let mut line_count = 0;
    let lines = dst.len() - 1;
    for i in dst {
        if indent {
            for _ in 0..pos_str.len() {
                output.push_str(" ");
            }
        }
        indent = true;
        output.push_str("[ ");
        for v in i {
            output.push_str(format!("{:02X} ", v).as_str());
        }
        if i.len() < 4 {
            let n = 4 - i.len();
            for _ in 0..n {
                output.push_str("   ");
            }
        }
        if line_count == lines {
            output.push_str("] -> ");
        } else {
            output.push_str("]\n");
        }
        line_count += 1;
    }
    output
}
