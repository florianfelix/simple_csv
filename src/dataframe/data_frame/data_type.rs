use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::DataValue;

#[derive(Debug, Clone)]
pub enum DataTypeParseError {
    StringNotParseableAsBool(String),
    ParseIntError(String),
    ParseFloatError(String),
    ParseDateError(String),
}

impl std::error::Error for DataTypeParseError {}
impl std::fmt::Display for DataTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DataType {
    Null,
    Bool,
    Int,
    Float,
    String,
    Date,
}

impl From<&DataValue> for DataType {
    fn from(value: &DataValue) -> Self {
        match value {
            DataValue::Null => DataType::Null,
            DataValue::Bool(_) => DataType::Bool,
            DataValue::Float(_) => DataType::Float,
            DataValue::Int(_) => DataType::Int,
            DataValue::String(_) => DataType::String,
            DataValue::Date(_) => DataType::Date,
        }
    }
}

impl DataType {
    pub fn parse(&self, value: &str) -> Result<DataValue, DataTypeParseError> {
        match self {
            DataType::Null => Ok(DataValue::Null),
            DataType::Bool => match value {
                "true" => Ok(DataValue::Bool(true)),
                "false" => Ok(DataValue::Bool(false)),
                _ => Err(DataTypeParseError::StringNotParseableAsBool(
                    value.to_owned(),
                )),
            },
            DataType::Int => match value.parse::<i64>() {
                Ok(v) => Ok(DataValue::Int(v)),
                Err(e) => Err(DataTypeParseError::ParseIntError(e.to_string())),
            },
            DataType::Float => match value.parse::<f64>() {
                Ok(v) => Ok(DataValue::Float(v.into())),
                Err(e) => Err(DataTypeParseError::ParseFloatError(e.to_string())),
            },
            DataType::String => match value.is_empty() {
                true => Ok(DataValue::Null),
                false => Ok(DataValue::String(value.to_owned())),
            },
            DataType::Date => match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
                Ok(v) => Ok(DataValue::Date(v)),
                Err(e) => Err(DataTypeParseError::ParseDateError(e.to_string())),
            },
        }
    }
}
