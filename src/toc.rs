// Internal module used to interpret ToC (Table of Contents) bitmasks.
//
// The lead in of the TDMS file (segment) contains a Table of Contents (ToC)
// which indicates what kind of data the segment contains.
//
// Any combination of the following flags can be encoded in the ToC:
//
// | Name               | Flag    | Description                                                                                                                          |
// | ------------------ | ------- | ------------------------------------------------------------------------------------------------------------------------------------ |
// | TocMetaData        | (1L<<1) | Segment contains meta data                                                                                                           |
// | TocRawData         | (1L<<3) | Segment contains raw data                                                                                                            |
// | TocDAQmxRawData    | (1L<<7) | Segment contains DAQmx raw data                                                                                                      |
// | TocInterleavedData | (1L<<5) | Raw data in the segment is interleaved (if flag is not set, data is contiguous)                                                      |
// | TocBigEndian       | (1L<<6) | All numeric values in the segment are big-endian formatted (if flag is not set, data is little-endian). ToC is always little-endian. |
// | TocNewObjList      | (1L<<2) | Segment contains new object list (e.g. channels in this segment are not the same channels the previous segment contains)             |

use std::fmt;

const TOC_FLAGS:[(Flag, i32); 6] = [
    (Flag::MetaData, 1 << 1),
    (Flag::RawData, 1 << 3),
    (Flag::DAQmxRawData, 1 << 7),
    (Flag::InterleavedData, 1 << 5),
    (Flag::BigEndian, 1 << 6),
    (Flag::NewObjList, 1 << 2),
];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Flag{
    MetaData,
    RawData,
    DAQmxRawData,
    InterleavedData,
    BigEndian,
    NewObjList,
}

impl fmt::Display for Flag{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self{
            Flag::MetaData => "MetaData",
            Flag::RawData => "RawData",
            Flag::DAQmxRawData => "DAQmxRawData",
            Flag::InterleavedData => "InterleavedData",
            Flag::BigEndian => "BigEndian",
            Flag::NewObjList => "NewObjList",
        };
        write!(f, "{}", printable)
    }
}

pub fn get_flags(mask: &i32) -> Vec<Flag>{
    let mut flags:Vec<Flag> = Vec::new();

    for (k, v) in TOC_FLAGS{
        if mask & v != 0{
            flags.push(k);
        }
    };

    flags
}
