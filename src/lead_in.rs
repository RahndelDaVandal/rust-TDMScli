#[derive(Debug)]
pub struct LeadIn {
    tag: String,
    toc: i32,
    ver: u32,
    seg: u64,
    raw: u64,
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
