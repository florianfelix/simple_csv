use serde::{Deserialize, Serialize};

use super::data_type::DataType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    name: String,
    dtype: DataType,
}
impl Header {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            dtype: DataType::Null,
        }
    }
    pub fn with_dtype(mut self, dtype: DataType) -> Self {
        self.dtype = dtype;
        self
    }
    pub fn dtype(&self) -> &DataType {
        &self.dtype
    }
    pub fn set_dtype(&mut self, dtype: DataType) {
        self.dtype = dtype;
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
    pub fn to_debug(&self) -> String {
        format!("{:?}({:})", &self.dtype, &self.name)
    }
}
impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{:?}>", self.name, self.dtype)
    }
}
