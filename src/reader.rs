use std::{
    fs::File,
    io::{
        BufReader,
        Read,
    }
};
use byteorder::{ByteOrder, LittleEndian};
use crate::data;
use crate::data::{Type, Value};

#[derive(Debug)]
pub struct Reader {
    pub file_len: i64,
    pub reader: BufReader<File>,
    pub buffer: Vec<u8>,
    pub last_pos: i64,
    pub pos: i64,
}

impl Reader{
    pub fn new(file_path: &String) -> Reader {
        match File::open(file_path) {
            Ok(file) => {
                Reader {
                    file_len: file.metadata().unwrap().len() as i64,
                    reader: BufReader::new(file),
                    buffer: Vec::new(),
                    last_pos: 0,
                    pos: 0,
                }
            },
            Err(e) => panic!("Problem creating Reader: {}", e),
        }
    }
    pub fn read(&mut self, n: i64) -> Vec<u8>{
        let mut buf = vec![0u8; n as usize];
        match self.reader.read(&mut buf) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {e}"),
        };
        self.buffer = buf.clone();
        self.last_pos = self.pos;
        self.pos += n;
        buf
    }
    pub fn read_u32(&mut self) -> u32{
        self.read(4);
        LittleEndian::read_u32(&self.buffer)
    }
    pub fn read_i32(&mut self) -> i32{
        self.read(4);
        LittleEndian::read_i32(&self.buffer)
    }
    pub fn read_u64(&mut self) -> u64{
        self.read(8);
        LittleEndian::read_u64(&self.buffer)
    }
    pub fn read_i64(&mut self) -> i64{
        self.read(8);
        LittleEndian::read_i64(&self.buffer)
    }
    pub fn read_string(&mut self, len: u32) -> String {
        self.read(len as i64);
        String::from_utf8_lossy(&self.buffer).to_string()
    }
    pub fn read_dvalue(&mut self, dtype: data::Type, str_len: Option<u32>) -> data::Value {
        match dtype {
            data::Type::Uint8 => {
                Value::get(dtype, &self.read(1))
            },
            data::Type::Int8 => {
                Value::get(dtype, &self.read(1))
            },
            data::Type::Uint16 => {
                Value::get(dtype, &self.read(2))
            },
            data::Type::Int16 => {
                Value::get(dtype, &self.read(2))
            },
            data::Type::Uint32 => {
                Value::get(dtype, &self.read(4))
            },
            data::Type::Int32 => {
                Value::get(dtype, &self.read(4))
            },
            data::Type::Uint64 => {
                Value::get(dtype, &self.read(8))
            },
            data::Type::Int64 => {
                Value::get(dtype, &self.read(8))
            },
            data::Type::Float => {
                Value::get(dtype, &self.read(4))
            },
            data::Type::Double => {
                Value::get(dtype, &self.read(8))
            },
            data::Type::String => {
                match str_len {
                    Some(l) => {
                        Value::get(dtype, &self.read(l as i64))
                    },
                    None => {
                        panic!(
                            "reader::Reader::read_dvalue Error: no string length provided."
                        );
                    }
                }
            },
            data::Type::Boolean => {
                Value::get(dtype, &self.read(1))
            },
            data::Type::TimeStamp => {
                Value::get(dtype, &self.read(16))
            },
            _ => {
                log::error!("unexpected data::Type: {dtype}");
                panic!("reader::Reader::read_dvalue Error: unexpected data::Type");
            },
        }
    }
    pub fn seek(&mut self, idx: i64) {
        match self.reader.seek_relative(idx as i64) {
            Ok(_) => {
                self.last_pos = self.pos;
                self.pos += idx as i64
            },
            Err(e) => eprintln!("Error: {e}"),
        }
    }
    pub fn rewind(&mut self){
        self.seek(- &self.pos);
    }
    pub fn get_segment_positions(&mut self) -> Vec<i64> {
        let mut segment_positions:Vec<i64> = Vec::new();

        while self.pos < self.file_len{
            segment_positions.push(self.pos);
            self.seek(12);
            let next_segment_offset =  self.read_u64();
            self.seek(next_segment_offset as i64 + 8);
        }

        segment_positions
    }
    pub fn get_num_of_objs(&mut self) -> u32 {
        let dbg = |b:&Vec<u8>, p:&i64| { if b.len() > 0{ dbg_bytes(b, p); } };

        let num_of_objs = self.read_u32();
        dbg(&self.buffer, &self.last_pos);
        println!("num_of_objs: {num_of_objs}");

        num_of_objs
    }
    pub fn read_obj(&mut self) -> u32 {
        let fmt_len_data_index = |x:u32| {
            if x == 0xFFFFFFFF {
                "None".to_string()
            } else {
                x.to_string()
            }
        };
        let dbg = |b:&Vec<u8>, p:&i64| { if b.len() > 0{ dbg_bytes(b, p); } };

        let path_len = self.read_u32();
        dbg(&self.buffer, &self.last_pos);
        println!("path_len: {path_len}");
        let path = self.read_string(path_len);
        dbg(&self.buffer, &self.last_pos);
        println!("path: \"{path}\"");
        let len_data_index = self.read_u32();
        dbg(&self.buffer, &self.last_pos);

        match len_data_index {
            crate::NO_RAW_DATA => {
                println!("len_data_index: {}", fmt_len_data_index(len_data_index));
            },
            _ => {
                println!("len_data_index: {}", fmt_len_data_index(len_data_index));
                let dtype = data::Type::get(self.read_u32());
                dbg(&self.buffer, &self.last_pos);
                println!("dtype: {}", dtype);
                let dimension = self.read_u32();
                dbg(&self.buffer, &self.last_pos);
                println!("dimension: {dimension}");
                let num_of_values = self.read_u64();
                dbg(&self.buffer, &self.last_pos);
                println!("num_of_values: {num_of_values}");
                match dtype {
                    Type::String => {
                        let size_of_values = self.read_u64();
                        dbg(&self.buffer, &self.last_pos);
                        println!("size_of_values: {size_of_values}");
                    },
                    _ => {},
                }
            }
        }

        let num_of_props = self.read_u32();
        dbg(&self.buffer, &self.last_pos);
        println!("num_of_props: {num_of_props}\n");
        num_of_props
    }
    pub fn read_prop(&mut self) {
        let dbg = |b:&Vec<u8>, p:&i64| { if b.len() > 0{ dbg_bytes(b, p); } };

        let name_len = self.read_u32();
        dbg(&self.buffer, &self.last_pos);
        println!("name_len: {name_len}");
        if name_len < 1 { return; }
        let name = self.read_string(name_len);
        dbg(&self.buffer, &self.last_pos);
        println!("name: \"{name}\"");
        let dtype = data::Type::get(self.read_u32());
        dbg(&self.buffer, &self.last_pos);
        println!("dtype: {dtype}");
        match dtype {
            Type::String => {
                let str_len = self.read_u32();
                dbg(&self.buffer, &self.last_pos);
                println!("str_len: {str_len}");
                let value = self.read_dvalue(dtype, Some(str_len));
                dbg(&self.buffer, &self.last_pos);
                println!("value: {value:?}");
            },
            _ => {
                let value = self.read_dvalue(dtype, None);
                dbg(&self.buffer, &self.last_pos);
                println!("value: {value:?}");
            },
        }
    }
    pub fn read_meta(&mut self) {
        let num_of_objs = self.get_num_of_objs();

        for _ in 0..num_of_objs {
            let num_of_props = self.read_obj();

            for _ in 0..num_of_props {
                self.read_prop();
            }
            println!();
        }
    }
}

pub fn dbg_bytes(src: &[u8], pos: &i64) {
    let mut output = String::new();
    let dst = src.chunks(4);
    let pos_str = format!("{pos:06}: ");
    output.push_str(&pos_str);
    let mut indent = false;
    let lines = dst.len() - 1;
    for (line_count, i) in dst.enumerate(){
        if indent {
            for _ in 0..pos_str.len() {
                output.push(' ');
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
    }
    print!("{}", output);
}
