mod de;
mod float;
mod ser;
pub use float::Float;

use super::DataType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DataValue {
    Null,
    String(String),
    Float(Float),
    Int(i64),
    Bool(bool),
}
impl DataValue {
    pub fn new(v: impl Into<DataValue>) -> Self {
        v.into()
    }
    pub fn dtype(&self) -> DataType {
        <&DataValue as Into<DataType>>::into(self)
    }
    pub fn print(&self) -> String {
        match self {
            DataValue::Null => String::new(),
            DataValue::String(v) => v.to_owned(),
            DataValue::Float(float) => float.to_string(),
            DataValue::Int(v) => v.to_string(),
            DataValue::Bool(v) => v.to_string(),
        }
    }
    pub fn convert_dtype(&mut self, dtype: DataType) {
        match dtype {
            DataType::Null => *self = DataValue::Null,
            DataType::String => *self = DataValue::String(self.print()),
            DataType::Float => *self = dtype.parse(&self.print()).unwrap_or(DataValue::Null),
            DataType::Int => *self = dtype.parse(&self.print()).unwrap_or(DataValue::Null),
            DataType::Bool => *self = dtype.parse(&self.print()).unwrap_or(DataValue::Null),
        }
    }
}

impl std::fmt::Display for DataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.print())
    }
}

impl<T> From<Option<T>> for DataValue
where
    T: Into<DataValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => DataValue::Null,
        }
    }
}
impl<T, E> From<Result<T, E>> for DataValue
where
    T: Into<DataValue>,
    E: std::error::Error,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(_e) => DataValue::Null,
        }
    }
}

impl From<String> for DataValue {
    fn from(value: String) -> Self {
        match value.is_empty() {
            true => Self::Null,
            false => Self::String(value),
        }
    }
}
impl From<&str> for DataValue {
    fn from(value: &str) -> Self {
        match value.is_empty() {
            true => Self::Null,
            false => Self::String(value.to_owned()),
        }
    }
}
impl From<f32> for DataValue {
    fn from(value: f32) -> Self {
        Self::Float(value.into())
    }
}
impl From<f64> for DataValue {
    fn from(value: f64) -> Self {
        Self::Float(value.into())
    }
}
impl From<i32> for DataValue {
    fn from(value: i32) -> Self {
        Self::Int(value as i64)
    }
}
impl From<i64> for DataValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<bool> for DataValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

// FLOAT
