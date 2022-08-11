use crate::dtype::Dtype;
use std::any::Any;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub struct MetaData{
    num_of_objs: u32,
    objects: Vec<Object>,
}

impl MetaData{
    pub fn new() -> Self{
        Self{
            num_of_objs: 0,
            objects: vec![],
        }
    }
    pub fn num_of_objs(&self) -> &u32{
        &self.num_of_objs
    }
    pub fn set_num_of_objs(&mut self, num: &[u8]){
        self.num_of_objs = LittleEndian::read_u32(num)
    }
    pub fn objects(&self) -> &Vec<Object>{
        &self.objects
    }
    pub fn add_object(&mut self, obj: Object){
        self.objects.push(obj)
    }
}

#[derive(Debug)]
pub struct Object{
    pub path: String,
    pub raw_index: u32,
    pub num_of_properties: u32,
    pub properties: Vec<Property>,
}

impl Object{
    pub fn add_property(&mut self, property: Property){
        self.properties.push(property)
    }
}

#[derive(Debug)]
pub struct Property{
    pub name: String,
    pub value: Dtype,
}
