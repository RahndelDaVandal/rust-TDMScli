use byteorder::{ByteOrder, LittleEndian};
use std::fmt;
#[derive(Debug)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float, // SingleFloat
    Double, // DoubleFloat
    String,
    Boolean,
    TimeStamp,
    TypeError,
//  ExtendedFloat=11
//  SingleFloatWithUnit=0x19        size=4    
//  DoubleFloatWithUnit=0x1A        size=8 
//  ExtendedFloatWithUnit=0x1B
//  FixedPoint=0x4F
//  ComplexSingleFloat=0x08000c     size=8
//  ComplexDoubleFloat=0x10000d     size=16
//  DAQmxRawData=0xFFFFFFFF
}
impl Type {
    pub fn get(value: u32) -> Self {
        match value {
            0x01 => Self::Int8,
            0x02 => Self::Int16,
            0x03 => Self::Int32,
            0x04 => Self::Int64,
            0x05 => Self::Uint8,
            0x06 => Self::Uint16,
            0x07 => Self::Uint32,
            0x08 => Self::Uint64,
            0x09 => Self::Float,
            0x0A => Self::Double,
            0x20 => Self::String,
            0x21 => Self::Boolean,
            0x44 => Self::TimeStamp,
            _ => {
                log::error!("unrecognized data::Type: {value}");
                Self::TypeError
            },
        }
    }
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match *self {
            Type::Int8 => "Int8".to_string(),
            Type::Int16 => "Int16".to_string(),
            Type::Int32 => "Int32".to_string(),
            Type::Int64 => "Int64".to_string(),
            Type::Uint8 => "Uint8".to_string(),
            Type::Uint16 => "Uint16".to_string(),
            Type::Uint32 => "Uint32".to_string(),
            Type::Uint64 => "Uint64".to_string(),
            Type::Float => "Float".to_string(),
            Type::Double => "Double".to_string(),
            Type::String => "String".to_string(),
            Type::Boolean => "Boolean".to_string(),
            Type::TimeStamp => "TimeStamp".to_string(),
            _ => "TypeError".to_string(),
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum Value {
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
    Boolean(bool),
    TimeStamp((i64, u64)),
    ValueError,
}
// TODO -
// Timestamps in TDMS files are stored as a structure of two components:
//     - (i64) seconds: since the epoch 01/01/1904 00:00:00.00 UTC (using the Gregorian calendar
//         and ignoring leap seconds)
//     - (u64) positive fractions: (2^-64) of a second
// Boolean values are stored as 1 byte each, where 1 represents TRUE and 0 represents FALSE.
impl Value {
    pub fn get(dtype: Type, buf: &[u8]) -> Self {
        let len_check = |x| {
            let l = buf.len();
            if l != x {
                panic!("data::Value::Error: unexpected buf length. Have {l}. Want {x}")
            }
        };
        match dtype {
            Type::Uint8 => Self::Uint8({
                len_check(1);
                buf[0]
            }),
            Type::Int8 => Self::Int8({
                len_check(1);
                buf[0] as i8
            }),
            Type::Uint16 => Self::Uint16({
                len_check(2);
                LittleEndian::read_u16(buf)
            }),
            Type::Int16 => Self::Int16({
                len_check(2);
                LittleEndian::read_i16(buf)
            }),
            Type::Uint32 => Self::Uint32({
                len_check(4);
                LittleEndian::read_u32(buf)
            }),
            Type::Int32 => Self::Int32({
                len_check(4);
                LittleEndian::read_i32(buf)
            }),
            Type::Uint64 => Self::Uint64({
                len_check(8);
                LittleEndian::read_u64(buf)
            }),
            Type::Int64 => Self::Int64({
                len_check(8);
                LittleEndian::read_i64(buf)
            }),
            Type::Float => Self::Float({
                len_check(4);
                LittleEndian::read_f32(buf)
            }),
            Type::Double => Self::Double({
                len_check(8);
                LittleEndian::read_f64(buf)
            }),
            Type::String => Self::String({ String::from_utf8_lossy(buf) }.to_string()),
            Type::Boolean => Self::Boolean({
                len_check(1);
                let val = buf[0];
                if val == 0 {
                    false
                } else if val == 1 {
                    true
                } else {
                    panic!(
                    "data::Value::Boolean Error: unexpected value. Want: 0 or 1. Got: {val}"
                     )
                }
            }),
            Type::TimeStamp => Self::TimeStamp({
                len_check(16);
                (
                    LittleEndian::read_i64(&buf[0..8]),
                    LittleEndian::read_u64(&buf[8..]),
                )
            }),
            _ => Self::ValueError,
        }
    }
}
// typedef enum {
//     tdsTypeVoid,
//     tdsTypeI8,    
//     tdsTypeI16,    
//     tdsTypeI32,    
//     tdsTypeI64,
//     tdsTypeU8,    
//     tdsTypeU16,    
//     tdsTypeU32,    
//     tdsTypeU64,
//     tdsTypeSingleFloat,    
//     tdsTypeDoubleFloat,    
//     tdsTypeExtendedFloat=11    
//     tdsTypeSingleFloatWithUnit=0x19,    
//     tdsTypeDoubleFloatWithUnit=0x1A,    
//     tdsTypeExtendedFloatWithUnit=0x1B,
//     tdsTypeString=0x20,   
//     tdsTypeBoolean=0x21,   
//     tdsTypeTimeStamp=0x44,
//     tdsTypeFixedPoint=0x4F,
//     tdsTypeComplexSingleFloat=0x08000c,
//     tdsTypeComplexDoubleFloat=0x10000d,
//     tdsTypeDAQmxRawData=0xFFFFFFFF
// } tdsDataType;
