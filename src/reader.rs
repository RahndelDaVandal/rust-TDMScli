use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use crate::lead_in::LeadIn;
use crate::toc::get_flags;

#[derive(Debug)]
pub struct Reader {
    pub reader: BufReader<File>,
    pub location: i32,
}

impl Reader {
    pub fn new(file_path: PathBuf) -> Self {
        let file = File::open(file_path).expect("Error opening file");
        Self {
            reader: BufReader::new(file),
            location: 0,
        }
    }
    pub fn read_next(&mut self, num: u32) -> Vec<u8> {
        let mut buf = vec![0u8; num as usize];

        match self.reader.read(&mut buf) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {e}"),
        };
        self.location += num as i32;

        buf
    }
    pub fn move_to(&mut self, idx: u64){
        match self.reader.seek_relative(idx as i64){
            Ok(_) => self.location += idx as i32,
            Err(e) => eprintln!("Error: {e}")
        }
    }
    pub fn read_lead_in(&mut self) -> LeadIn {
        let mut buf = self.read_next(4);
        let tag = String::from_utf8_lossy(&buf).to_string();

        buf = self.read_next(4);
        let toc = get_flags(&LittleEndian::read_i32(&buf));

        buf = self.read_next(4);
        let ver = LittleEndian::read_u32(&buf);

        buf = self.read_next(8);
        let seg = LittleEndian::read_u64(&buf);

        buf = self.read_next(8);
        let raw = LittleEndian::read_u64(&buf);

        LeadIn{tag, toc, ver, seg, raw}
    }
}
