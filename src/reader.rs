use crate::toc::get_flags;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

use crate::segment::{Segment, LeadIn, Object, Property};
use crate::dtype::{Dtype, get_dtype_as_string, get_val_by_dtype};

const NO_RAW_DATA:u32 = 0xFFFFFFFF;
const DAQMX_FORMAT_CHANGING_SCALER:u32 = 0x69120000;
const DAQMX_DIGITAL_LINE_SCALER:u32 = 0x69130000;

#[derive(Debug)]
pub struct Reader {
    pub reader: BufReader<File>,
    pub buffer: Vec<u8>,
    pub location: i32,
}

impl Reader {
    pub fn new(file_path: PathBuf) -> Self {
        let file = File::open(file_path).expect("Error opening file");
        Self {
            reader: BufReader::new(file),
            buffer: Vec::new(),
            location: 0,
        }
    }
    pub fn read_next(&mut self, num: u32) -> Vec<u8> {
        let mut buf = vec![0u8; num as usize];

        match self.reader.read(&mut buf) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {e}"),
        };
        self.buffer = buf.clone();
        self.location += num as i32;

        buf
    }
    pub fn move_to(&mut self, idx: u64) {
        match self.reader.seek_relative(idx as i64) {
            Ok(_) => self.location += idx as i32,
            Err(e) => eprintln!("Error: {e}"),
        }
    }

    pub fn read_segment(&mut self) -> Segment{
        // Position
        let position = self.location;
        for _ in 0..122{print!("=");} println!("");
        println!("SEGMENT AT POSITION {position}");
        for _ in 0..122{print!("-");} println!("");

        // LeadIn
        let lead_in = self.get_lead_in();
        for _ in 0..122{print!("-");} println!("");

        // num_of_objs
        let num_of_objs = self.get_num_objs();
        for _ in 0..122{print!("-");} println!("");

        let mut objects:Vec<Object> = Vec::new();
        for i in 0..num_of_objs{
            let obj = self.get_obj();
            objects.push(obj);
            if i != num_of_objs - 1{
                for _ in 0..122{print!("-");} println!("");
            }
        }

        Segment {
            position,
            lead_in,
            num_of_objs,
            objects,
        }
    }

    pub fn get_lead_in(&mut self) -> LeadIn{
        let tag = String::from_utf8_lossy(&self.read_next(4)).to_string();
        print!("{}", self.bytes_formatter());
        println!("TAG: \"{tag}\"");

        let toc_mask = LittleEndian::read_i32(&self.read_next(4));
        let toc_flags = get_flags(&toc_mask);
        print!("{}", self.bytes_formatter());
        print!("TOC_FLAGS: [ ");
        for f in &toc_flags{
            print!("{} ", f);
        }
        println!("]");

        let version = LittleEndian::read_u32(&self.read_next(4));
        print!("{}", self.bytes_formatter());
        println!("VERSION: {version}");

        let next_segment_offset = LittleEndian::read_u64(&self.read_next(8));
        print!("{}", self.bytes_formatter());
        println!("NEXT_SEGMENT_OFFSET: {next_segment_offset}");
        
        let raw_data_offset = LittleEndian::read_u64(&self.read_next(8));
        print!("{}", self.bytes_formatter());
        println!("RAW_DATA_OFFSET: {raw_data_offset}");

        LeadIn {
            tag,
            toc_flags,
            version,
            next_segment_offset,
            raw_data_offset,
        }
    }

    pub fn get_obj(&mut self) -> Object{
        let mut properties:Vec<Property> = Vec::new();

        let path = self.get_path();
        let data_index = self.get_raw_index();

        match data_index {
            NO_RAW_DATA => {
                let num_of_properties = self.get_num_of_props();
                if num_of_properties > 0 {
                    for _ in 0..num_of_properties{
                        let prop = self.get_prop();
                        properties.push(prop);
                    }
                }
                return Object {
                    path,
                    data_index,
                    data_dtype: None,
                    data_dimension: None,
                    num_of_data_values: None,
                    num_of_properties,
                    properties,
                };
            },
            // TODO -
            DAQMX_FORMAT_CHANGING_SCALER => {
                // TODO - Work out logic
                panic!("NOT IMPLIMENTED: data_index == DAQMX_FORMAT_CHANGING_SCALER");
            },
            DAQMX_DIGITAL_LINE_SCALER => {
                // TODO - Work out logic
                panic!("NOT IMPLIMENTED: data_index == DAQMX_DIGITAL_LINE_SCALER");
            },
            0x0000000 => {
                // TODO - Work out logic
                // data_index is same as previous object
                panic!("NOT IMPLIMENTED: data_index == 0x0000000");
            }
            _ => {
                let data_dtype = self.get_data_dtype();
                let data_dimension = self.get_data_dimension();
                let num_of_data_values = self.get_num_of_data_values();
                let num_of_properties = self.get_num_of_props();
                if num_of_properties > 0 {
                    for _ in 0..num_of_properties{
                        let prop = self.get_prop();
                        properties.push(prop);
                    }
                }
                return Object {
                    path,
                    data_index,
                    data_dtype: Some(data_dtype),
                    data_dimension: Some(data_dimension),
                    num_of_data_values: Some(num_of_data_values),
                    num_of_properties,
                    properties,
                };
            },
        }
    }

    pub fn get_num_objs(&mut self) -> u32{
        let num_of_objs = LittleEndian::read_u32(&self.read_next(4));

        print!("{}", self.bytes_formatter());
        println!("#_OF_OBJECTS: {num_of_objs}");

        return num_of_objs;
    }
    pub fn get_path(&mut self) -> String {
        let path_len = LittleEndian::read_u32(&self.read_next(4));
        if path_len > 1000 as u32 {
            panic!("WARNING! - Path Length is greater than 1000! ({path_len})");
        }
        let path = String::from_utf8_lossy(&self.read_next(path_len)).to_string();

        print!("{}", self.bytes_formatter());
        println!("PATH: \"{path}\"");

        return path;
    }
    pub fn get_raw_index(&mut self) -> u32{
        let raw_data_index = LittleEndian::read_u32(&self.read_next(4));

        print!("{}", self.bytes_formatter());

        match raw_data_index{
            NO_RAW_DATA => {
                println!("RAW_DATA_INDEX: NONE");
            },
            DAQMX_FORMAT_CHANGING_SCALER => {
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}");
            },
            DAQMX_DIGITAL_LINE_SCALER => {
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}");
            },
            0x0000000 => {
                // INDEX MATCHES PERVOUS OBJECT RAW_DATA_INDEX
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}");
            },
            _ => {
                println!("RAW_DATA_INDEX: {raw_data_index}");
            },
        }
        return raw_data_index;
    }
    pub fn get_num_of_props(&mut self) -> u32{
        let num_of_props = LittleEndian::read_u32(&self.read_next(4));

        print!("{}", self.bytes_formatter());
        println!("#_OF_PROPS: {num_of_props}\n");

        return num_of_props;
    }
    pub fn get_prop(&mut self) -> Property{
        let name_len = LittleEndian::read_u32(&self.read_next(4));
        print!("{}", self.bytes_formatter());
        println!("PROP_NAME_LEN: {name_len}");

        let name = if name_len <= 0{
            // panic!("ERROR!: reader.Reader.get_prop.name_len <= 0 ({})", name_len);
            "DBG_PROP_NAME_LEN_EQ_0".to_string()
        } else if name_len < 1000 && name_len > 0{
            String::from_utf8_lossy(&self.read_next(name_len)).to_string()
        } else{
            panic!("ERROR!: reader.Reader.get_prop.name_len > 1000 ({})", name_len);
        };

        // let name = String::from_utf8_lossy(&self.read_next(name_len)).to_string();

        print!("{}", self.bytes_formatter());
        println!("PROP_NAME: {name}");

        let dtype_u32 = LittleEndian::read_u32(&self.read_next(4));
        let dtype_str = get_dtype_as_string(dtype_u32);
        print!("{}", self.bytes_formatter());
        println!("PROP_DTYPE: {dtype_str}");

        match dtype_u32{
            0x20 => {
                let prop_value_len = LittleEndian::read_u32(&self.read_next(4));
                print!("{}", self.bytes_formatter());
                println!("PROP_VALUE_LEN: {prop_value_len}");
                let mut value = Dtype::String("".to_string());

                if prop_value_len > 0 {
                    value = get_val_by_dtype(dtype_u32, &self.read_next(prop_value_len));
                } 
                
                print!("{}", self.bytes_formatter());
                println!("PROP_Value: {value:?}\n");

                Property{
                    name,
                    dtype: dtype_str,
                    value,
                }
            },
            _ => {
                let value = get_val_by_dtype(dtype_u32, &self.read_next(4));
                print!("{}", self.bytes_formatter());
                println!("PROP_Value: {value:?}\n");

                Property{
                    name,
                    dtype: dtype_str,
                    value,
                }
            },
        }
    }
    pub fn get_data_dtype(&mut self) -> String{
        let dtype_u32 = LittleEndian::read_u32(&self.read_next(4));
        let dtype_str = get_dtype_as_string(dtype_u32);
        print!("{}", self.bytes_formatter());
        println!("DATA_DTYPE: {dtype_str}");
        return dtype_str;
    }
    pub fn get_data_dimension(&mut self) -> u32{
        let dimension = LittleEndian::read_u32(&self.read_next(4));
        print!("{}", self.bytes_formatter());
        println!("DATA_DIMENSION: {dimension}");
        return dimension;
    }
    pub fn get_num_of_data_values(&mut self) -> u64{
        let num_of_data_values = LittleEndian::read_u64(&self.read_next(8));
        print!("{}", self.bytes_formatter());
        println!("NUM_OF_DATA_VALUES: {num_of_data_values}");
        return num_of_data_values;
    }

    fn bytes_formatter(&self) -> String{
        let mut output = String::new();
        let dst = self.buffer.chunks(4);
        let pos = self.location;
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
