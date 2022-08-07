#[derive(Debug)]
pub struct LeadIn {
    pub tag: String,
    pub toc: i32,
    pub ver: u32,
    pub seg: u64,
    pub raw: u64,
}

impl LeadIn {
    pub fn new() -> Self {
        Self {
            tag: String::new(),
            toc: 0,
            ver: 0,
            seg: 0,
            raw: 0,
        }
    }
}
