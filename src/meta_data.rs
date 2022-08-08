use crate::dtype::{Dtype, get_dtype};
use std::any::Any;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub struct MetaData<T>{
    num_of_objs: u32,
    objects: Vec<Object<T>>,
}

impl<T> MetaData<T>{
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
    pub fn objects(&self) -> &Vec<Object<T>>{
        &self.objects
    }
    pub fn add_object(&mut self, obj: Object<T>){
        self.objects.push(obj)
    }
}

#[derive(Debug)]
pub struct Object<T>{
    len_of_path: u32,
    path: String,
    raw_index: u32,
    num_of_properties: u32,
    properties: Vec<Property<T>>,
}

impl<T> Object<T>{
    pub fn new() -> Self{
        Self{
            len_of_path: 0,
            path: String::new(),
            raw_index: 0,
            num_of_properties: 0,
            properties: vec![],
        }
    }
    pub fn len_of_path(&self) -> &u32{
        &self.len_of_path
    }
    pub fn set_len_of_path(&mut self, length: &[u8]){
        self.len_of_path = LittleEndian::read_u32(length)
    }
    pub fn path(&self) -> &str{
        &self.path
    }
    pub fn set_path(&mut self, path: &[u8]){
        self.path = String::from_utf8_lossy(path).to_string()
    }
    pub fn raw_index(&self) -> &u32{
        &self.raw_index
    }
    pub fn set_raw_index(&mut self, index: &[u8]){
        self.raw_index = LittleEndian::read_u32(index)
    }
    pub fn num_of_properties(&self) -> &u32{
        &self.num_of_properties
    }
    pub fn set_num_of_properties(&mut self, num: &[u8]){
        self.num_of_properties = LittleEndian::read_u32(num)
    }
    pub fn properties(&self) -> &Vec<Property<T>>{
        &self.properties
    }
    pub fn add_property(&mut self, property: Property<T>){
        self.properties.push(property)
    }
}

#[derive(Debug)]
pub struct Value;

#[derive(Debug)]
pub struct Property<T>{
    len_of_name: u32,
    name: String,
    dtype: Option<Dtype>,
    len_value: Option<u32>,
    value: Option<T>,
}

impl<T> Property<T>{
    pub fn new() -> Self{
        Self{
            len_of_name: 0,
            name: String::new(),
            dtype: None, 
            len_value: None,
            value: None,
        }
    }
    pub fn len_of_name(&self) -> &u32{
        &self.len_of_name
    }
    pub fn set_len_of_name(&mut self, length: &[u8]){
        self.len_of_name = LittleEndian::read_u32(length)
    }
    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn set_name(&mut self, name: &[u8]){
        self.name = String::from_utf8_lossy(name).to_string()
    }
    pub fn dtype(&self) -> &Option<Dtype>{
        &self.dtype
    }
    pub fn set_dtype(&mut self, dtype: &[u8]){
        let dtype_num = LittleEndian::read_u32(dtype);
        self.dtype = Some(get_dtype(dtype_num))
    }
    pub fn len_value(&self) -> &Option<u32>{
        &self.len_value
    }
    pub fn set_len_value(&mut self, length: &[u8]){
        self.len_value = Some(LittleEndian::read_u32(length))
    }
    pub fn value(&self) -> &Option<T>{
        &self.value
    }
    pub fn set_value(&mut self, val: &[u8]){
        // match self.dtype{
        //     Some(Dtype::String) => {self.value = Some(*String::from_utf8_lossy(val))},
        //     _ => {println!("Error")},
        // };
    }
}
