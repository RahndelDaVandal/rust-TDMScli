use crate::toc::Flag;

#[derive(Debug)]
pub struct LeadIn {
    pub tag: String,
    pub toc: Vec<Flag>,
    pub ver: u32,
    pub seg: u64,
    pub raw: u64,
}
