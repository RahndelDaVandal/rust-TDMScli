use std::{
    fmt,
    fmt::Write,
    string::String,
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
    pub buffer: Buffer,
    pub last_pos: i64,
    pub pos: i64,
}
impl fmt::Display for Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut o = String::new();
        writeln!(&mut o, "tdms::reader::Reader")?;
        writeln!(&mut o, "\tfile_len: {}", self.file_len)?;
        writeln!(&mut o, "\tbuffer: {}", self.buffer)?;
        writeln!(&mut o, "\tlast_pos: {}", self.last_pos)?;
        writeln!(&mut o, "\tpos: {}", self.pos)?;
        write!(f, "{o}")
    }
}

impl Reader{
    pub fn new(file_path: &String) -> Reader {
        match File::open(file_path) {
            Ok(file) => {
                let file_len = file.metadata().unwrap().len();
                Reader {
                    file_len: file_len as i64,
                    reader: BufReader::with_capacity(file_len as usize, file),
                    buffer: Buffer::new(),
                    last_pos: 0,
                    pos: 0,
                }
            },
            Err(e) => panic!("Problem creating Reader: {}", e),
        }
    }
    pub fn read(&mut self, n: i64) -> Buffer{
        let mut buf = vec![0u8; n as usize];
        match self.reader.read(&mut buf) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {e}"),
        };
        self.buffer = Buffer(buf.clone());
        self.last_pos = self.pos;
        self.pos += n;
        Buffer(buf)
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
                        log::error!("Reader::read_dvalue Error: str_len {:?}", str_len);
                        log::error!("\n{}", self);
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
                log::error!("\n{}", self);
                log::error!("\n{:?}", self.reader);
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
        // let dbg = |b:&Vec<u8>, p:&i64| { if b.len() > 0{ dbg_bytes(b, p); } };
        log::debug!("Getting # of object in segment at position: {}", self.pos);

        let num_of_objs = self.read_u32();
        // dbg(&self.buffer, &self.last_pos);
        // println!("num_of_objs: {num_of_objs}");
        log::debug!("number of objects: {}", num_of_objs);
        num_of_objs
    }
    pub fn read_obj(&mut self) -> u32 {
        log::debug!("OBJECT METADATA AT POSITION {}", self.pos);

        let path_len = self.read_u32();
        log::debug!("path_len: {} | [{:X}]", path_len, path_len);
        let path = self.read_string(path_len);
        log::debug!("path: {} | {}", path, self.buffer);
        let len_data_index = self.read_u32();
        log::debug!("Index Header: [{:08X}]", len_data_index);

        match len_data_index {
            crate::NO_RAW_DATA => {
            },
            _ => {
                let dtype_bytes = self.read_u32();
                log::debug!("dtype bytes: {}", self.buffer);
                let dtype = data::Type::get(dtype_bytes);
                log::debug!("dtype: {}", dtype);
                let dimension = self.read_u32();
                log::debug!("dimension: {} | [{:08X}]", dimension, dimension);
                let num_of_values = self.read_u64();
                log::debug!("number of values: {} | [{:016X}]", num_of_values, num_of_values);
                match dtype {
                    Type::String => {
                        let size_of_values = self.read_u64();
                        log::debug!("value length: {} | [{:X}]", size_of_values, size_of_values);
                    },
                    _ => {},
                }
            }
        }

        let num_of_props = self.read_u32();
        log::debug!("number of properties: {}", num_of_props);
        num_of_props
    }
    pub fn read_prop(&mut self) {
        log::debug!("READING PROPERTY AT POSITION: {}", self.pos);

        let name_len = self.read_u32();
        log::debug!("name length: {} | [{:08X}]", name_len, name_len);
        if name_len < 1 { 
            log::debug!("name length < 0. moving to next property");
            return; 
        }
        let name = self.read_string(name_len);
        log::debug!("PROPERTY Name: '{name}'");
        let dtype = data::Type::get(self.read_u32());
        log::debug!("dtype: {} | {}", dtype, self.buffer);
        match dtype {
            Type::String => {
                let str_len = self.read_u32();
                let value = self.read_dvalue(dtype, Some(str_len));
                log::debug!("PROPERTY Value: '{:?}'", value);
            },
            _ => {
                let value = self.read_dvalue(dtype, None);
                log::debug!("PROPERTY Value: {:?}", value);
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

#[derive(Debug, Clone)]
pub struct Buffer(Vec<u8>);
impl Buffer {
    pub fn new() -> Self {
        let buffer: Vec<u8> = Vec::new();
        Buffer(buffer)
    }
}
impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
impl std::ops::Deref for Buffer {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut o = String::new();
        for i in self.iter() {
            write!(&mut o, "{:02X} ", i)?;
        }
        write!(f, "[{}]", o.trim())
    }
}
