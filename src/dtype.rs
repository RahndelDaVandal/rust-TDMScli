#[derive(Debug)]
pub enum Dtype {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float,
    Double,
    String,
    Boolean,
    TimeStamp,
    DtypeError,
}

pub fn get_dtype(value: u32) -> Dtype{
    match value{
        0x01 => Dtype::Int8,
        0x02 => Dtype::Int16,
        0x03 => Dtype::Int32,
        0x04 => Dtype::Int64,
        0x05 => Dtype::Uint8,
        0x06 => Dtype::Uint16,
        0x07 => Dtype::Uint32,
        0x08 => Dtype::Uint64,
        0x09 => Dtype::Float,
        0x0A => Dtype::Double,
        0x20 => Dtype::String,
        0x21 => Dtype::Boolean,
        0x44 => Dtype::TimeStamp,
        _ => Dtype::DtypeError,
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
