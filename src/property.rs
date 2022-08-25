use crate::data;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Property {
    name: String,
    dtype: data::Type,
    value: data::Value,
}
// impl Property.value() -> <Value based on get_value_by_dtype>
