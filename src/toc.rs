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


#[derive(Debug)]
pub enum ToC{
    TocMetaData = 1<<1,
    TocRawData = 1<<3,
    TocDAQmxRawData = 1<<7,
    TocInterleavedData = 1<<5,
    TocBigEndian = 1<<6,
    TocNewObjList = 1<<2,
}

// let m = LittleEndian::read_i32(&mask);
// for (k, v) in toc_hash{
//     println!("{k} ({v:08b} & {:08b} != 0) = {}", &m, (&m & v != 0))
// }
