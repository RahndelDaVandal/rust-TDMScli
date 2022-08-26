use std::fmt;
use std::fmt::Write;
use byteorder::{ByteOrder, LittleEndian};
use crate::toc::{Flag, get_flags};

#[derive(Debug)]
pub struct LeadIn{
    pub position: i64,
    pub tag: String,
    pub toc_flags: Vec<Flag>,
    pub version: u32,
    pub next_segment_offset: u64,
    pub raw_data_offset: u64,
}
impl LeadIn {
    pub fn new(buffer: &[u8], position: i64) -> LeadIn {
        let li = LeadIn {
            position,
            tag: String::from_utf8_lossy(&buffer[0..4]).to_string(),
            toc_flags: get_flags(&LittleEndian::read_i32(&buffer[4..8])),
            version: LittleEndian::read_u32(&buffer[8..12]),
            next_segment_offset: LittleEndian::read_u64(&buffer[12..20]),
            raw_data_offset: LittleEndian::read_u64(&buffer[20..28]),
        };
        log::debug!("\n{}", li);
        li
    }
}
impl fmt::Display for LeadIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut o = String::new();
        writeln!(&mut o, "LeadIn at position: {}", self.position)?;
        writeln!(&mut o, "\ttag: {}", self.tag)?;
        writeln!(&mut o, "\ttoc: [")?;
        for i in self.toc_flags.iter() {
            writeln!(&mut o, "\t\t{},", i)?;
        }
        writeln!(&mut o, "\t]")?;
        writeln!(&mut o, "\tversion: {}", self.version)?;
        writeln!(&mut o, "\tnext_segment: {}", self.next_segment_offset)?;
        writeln!(&mut o, "\traw_data: {}", self.raw_data_offset)?;
        write!(f, "{}", o)
    }
}
