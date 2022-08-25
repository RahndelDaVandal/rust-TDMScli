use crate::property::Property;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Object {
    path: String,
    data_index: u32,
    num_of_properties: u32,
    properties: Vec<Property>,
}
