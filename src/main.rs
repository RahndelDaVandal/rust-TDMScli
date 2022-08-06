use std::env;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


fn main() {
    let mut file_path = PathBuf::new();
    match env::current_dir() {
        Ok(mut cwd) => {
            cwd.push("data");
            cwd.push("2020-09-17T22-45-47_.tdms");
            dbg!(&cwd);
            file_path = cwd
        }
        Err(e) => {
            println!("Error creating file_path: {}", e)
        }
    }
    dbg!(&file_path, &file_path.exists());

    // let file = File::open(file_path).expect("Error reading file");
    //
    // let mut reader = BufReader::new(file);
    // 
    // let mut tag = [0; 4];
    // read_bytes(&mut reader, &mut tag);
    // println!("tag: {}", String::from_utf8_lossy(&tag));
    //
    // let mut mask = [0; 4];
    // read_bytes(&mut reader, &mut mask);
    // println!("{:0<4b} {:0<4b} {:0<4b} {:0<4b}", mask[0], mask[1], mask[2], mask[3]);
    //
    // let mut ver = [0; 4];
    // read_bytes(&mut reader, &mut ver);
    // println!("ver: {:?}", u32::from_le_bytes(ver));
    //
    // let mut seg = [0; 8];
    // read_bytes(&mut reader, &mut seg);
    // println!("seg: {}", u64::from_le_bytes(seg));
    //
    // let mut raw = [0; 8];
    // read_bytes(&mut reader, &mut raw);
    // println!("raw: {}", u64::from_le_bytes(raw));

    let mut r = Reader::new(file_path);
    let tag = r.read_next(4);
    dbg!(String::from_utf8_lossy(&tag));
    let mask = r.read_next(4);
    dbg!(mask);
    let mut ver = r.read_next(4);
    dbg!(&ver);
}

#[derive(Debug)]
struct Reader{
    reader: BufReader<File>,
}

impl Reader{
    fn new(file_path: PathBuf) -> Self{
        let file = File::open(file_path).expect("Error opening file");
        Self{
            reader : BufReader::new(file),
        }
    }
    fn read_next(&mut self, num: usize) -> Vec<u8>{
        let mut buf = vec![0u8; num];
        match self.reader.read(&mut buf){
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        };
        buf
    }
}

#[derive(Debug)]
struct LeadIn{
    tag: String,
    mask: [u8; 4],
    ver: u32,
    seg: u64,
    raw: u64,
}

impl LeadIn{
    fn new() -> Self{
        Self {
            tag: String::new(),
            mask: [0; 4],
            ver:  0,
            seg: 0,
            raw: 0,
        }
    }
}

fn read_bytes(reader: &mut BufReader<File>, output: &mut [u8]){
    match reader.read(output){
        Ok(_) => {},
        Err(e) => eprintln!("Error: {}", e)
    };
}

// fn read_4bytes(file: &mut File) -> [u8; 4]{
//     let mut buffer = [0; 4];
//     let mut handle = file.take(4);
//     handle.read(&mut buffer).expect("error reading bytes");
//     return buffer
// }

// fn read_8bytes(file: &mut File) -> [u8; 8]{
//     let mut buffer = [0; 8];
//     let mut handle = file.take(8);
//     handle.read(&mut buffer).expect("error reading bytes");
//     return buffer
// }
