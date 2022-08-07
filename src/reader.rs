use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use crate::lead_in::LeadIn;

#[derive(Debug)]
pub struct Reader {
    reader: BufReader<File>,
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
    pub fn read_next(&mut self, num: usize) -> Vec<u8> {
        let mut buf = vec![0u8; num];

        match self.reader.read(&mut buf) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        };
        self.location += num as i32;

        buf
    }
    pub fn read_lead_in(&mut self) -> LeadIn {
        let mut lead_in = LeadIn::new();

        let mut buf = self.read_next(4);
        let tag = String::from_utf8_lossy(&buf);
        lead_in.tag = tag.into_owned();

        buf = self.read_next(4);
        lead_in.toc = LittleEndian::read_i32(&buf);

        buf = self.read_next(4);
        lead_in.ver = LittleEndian::read_u32(&buf);

        buf = self.read_next(8);
        lead_in.seg = LittleEndian::read_u64(&buf);

        buf = self.read_next(8);
        lead_in.raw = LittleEndian::read_u64(&buf);

        lead_in
    }
}
