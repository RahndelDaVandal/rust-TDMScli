use crate::toc::get_flags;
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

use crate::segment::Property;
use crate::dtype::{get_dtype_as_string, get_val_by_dtype};

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
    pub fn get_num_objs(&mut self) -> u32{
        let num_of_objs = LittleEndian::read_u32(&self.read_next(4));

        print!("{}", self.bytes_formatter());
        println!("#_OF_OBJECTS: {num_of_objs}\n");

        return num_of_objs;
    }
    pub fn get_path(&mut self) -> String {
        let path_len = LittleEndian::read_u32(&self.read_next(4));
        let path = String::from_utf8_lossy(&self.read_next(path_len)).to_string();

        print!("{}", self.bytes_formatter());
        println!("PATH: \"{path}\"\n");

        return path;
    }
    pub fn get_raw_index(&mut self) -> u32{
        let raw_data_index = LittleEndian::read_u32(&self.read_next(4));

        print!("{}", self.bytes_formatter());

        match raw_data_index{
            NO_RAW_DATA => {
                println!("RAW_DATA_INDEX: NONE\n");
            },
            DAQMX_FORMAT_CHANGING_SCALER => {
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}\n");
            },
            DAQMX_DIGITAL_LINE_SCALER => {
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}\n");
            },
            0x0000000 => {
                // INDEX MATCHES PERVOUS OBJECT RAW_DATA_INDEX
                // TODO - WORK OUT LOGIC
                println!("RAW_DATA_INDEX: {raw_data_index}\n");
            },
            _ => {
                println!("RAW_DATA_INDEX: {raw_data_index}\n");
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
        println!("PROP_NAME_LEN: {name_len}\n");

        let name = String::from_utf8_lossy(&self.read_next(name_len)).to_string();
        print!("{}", self.bytes_formatter());
        println!("PROP_NAME: {name}\n");

        let dtype_u32 = LittleEndian::read_u32(&self.read_next(4));
        let dtype_str = get_dtype_as_string(dtype_u32);
        print!("{}", self.bytes_formatter());
        println!("PROP_DTYPE: {dtype_str}\n");

        match dtype_u32{
            0x20 => {
                let prop_value_len = LittleEndian::read_u32(&self.read_next(4));
                print!("{}", self.bytes_formatter());
                println!("PROP_VALUE_LEN: {prop_value_len}\n");

                let value = get_val_by_dtype(dtype_u32, &self.read_next(prop_value_len));
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
        println!("DATA_DTYPE: {dtype_str}\n");
        return dtype_str;
    }
    pub fn get_data_dimension(&mut self) -> u32{
        let dimension = LittleEndian::read_u32(&self.read_next(4));
        print!("{}", self.bytes_formatter());
        println!("DATA_DIMENSION: {dimension}\n");
        return dimension;
    }
    pub fn get_num_of_data_values(&mut self) -> u64{
        let num_of_data_values = LittleEndian::read_u64(&self.read_next(8));
        print!("{}", self.bytes_formatter());
        println!("NUM_OF_DATA_VALUES: {num_of_data_values}\n");
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
