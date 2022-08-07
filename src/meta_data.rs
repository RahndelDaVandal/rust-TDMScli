use crate::dtype::Dtype;

#[derive(Debug)]
pub struct MetaData{
    pub num_of_obj: u32,
    pub objects: Vec<Object>,
}

impl MetaData{
    pub fn new() -> Self{
        Self{
            num_of_obj: 0,
            objects: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Object{
    pub len_of_path: u32,
    pub path: String,
    pub raw_index: u32,
    pub num_of_properties: u32,
    pub properties: Vec<Property>,
}

impl Object{
    pub fn new() -> Self{
        Self{
            len_of_path: 0,
            path: String::new(),
            raw_index: 0,
            num_of_properties: 0,
            properties: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Property{
    pub len_of_name: u32,
    pub name: String,
    pub dtype: Option<Dtype>,
    pub len_val: Option<u32>,
    // value: TODO - Generic Type or Option
}

impl Property{
    pub fn new() -> Self{
        Self{
            len_of_name: 0,
            name: String::new(),
            dtype: None, 
            len_val: None,
        }
    }
}
