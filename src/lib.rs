pub mod data;
pub mod leadin;
pub mod object;
pub mod property;
pub mod reader;
pub mod toc;

pub const NO_RAW_DATA: u32 = 0xFFFFFFFF;
pub const DAQMX_FORMAT_CHANGING_SCALER: u32 = 0x69120000;
pub const DAQMX_DIGITAL_LINE_SCALER: u32 = 0x69130000;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config { file_path })
    }
}

pub fn dbg_format_bytes(src: &[u8], pos: &i64) -> String {
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
    output
}
