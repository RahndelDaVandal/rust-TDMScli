use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub enum Dtype {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
    Double(f64),
    String(String),
    Boolean, // TODO - Figure out how Ni stores bools as &[u8]
    TimeStamp, // TODO - Figure out how to make timestamp
    DtypeError,
}

pub fn get_dtype(dtype: u32, value: &[u8]) -> Dtype{
    match dtype{
        0x01 => Dtype::Int8(LittleEndian::read_i16(value) as i8),
        0x02 => Dtype::Int16(LittleEndian::read_i16(value)),
        0x03 => Dtype::Int32(LittleEndian::read_i32(value)),
        0x04 => Dtype::Int64(LittleEndian::read_i64(value)),
        0x05 => Dtype::Uint8(LittleEndian::read_u16(value) as u8),
        0x06 => Dtype::Uint16(LittleEndian::read_u16(value)),
        0x07 => Dtype::Uint32(LittleEndian::read_u32(value)),
        0x08 => Dtype::Uint64(LittleEndian::read_u64(value)),
        0x09 => Dtype::Float(LittleEndian::read_f32(value)),
        0x0A => Dtype::Double(LittleEndian::read_f64(value)),
        0x20 => Dtype::String(String::from_utf8_lossy(value).to_string()),
        0x21 => Dtype::Boolean,
        0x44 => Dtype::TimeStamp,
        _ => {Dtype::DtypeError}
    }
}

// int8 = 0x01,
// int16 = 0x02,
// int32 = 0x03,
// int64 = 0x04,
// uint8 = 0x05,
// uint16 = 0x06,
// uint32 = 0x07,
// uint64 = 0x08,
// float = 0x09,
// double = 0x0A,
// string = 0x20,
// boolean = 0x21,
// timestamp = 0x44,

// #[derive(Debug)]
// pub enum Dtype {
//     Int8 = 0x01,
//     Int16 = 0x02,
//     Int32 = 0x03,
//     Int64 = 0x04,
//     Uint8 = 0x05,
//     Uint16 = 0x06,
//     Uint32 = 0x07,
//     Uint64 = 0x08,
//     Float = 0x09,
//     Double = 0x0A,
//     String = 0x20,
//     Boolean = 0x21,
//     TimeStamp = 0x44,
// }
