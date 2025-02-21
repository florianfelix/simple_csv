use serde::{Serialize, Serializer};

use super::DataValue;

impl Serialize for DataValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DataValue::Null => serializer.serialize_none(),
            DataValue::Int(value) => serializer.serialize_i64(value),
            DataValue::Float(value) => serializer.serialize_f64(*value),
            DataValue::String(ref value) => serializer.serialize_str(value),
            DataValue::Bool(value) => serializer.serialize_bool(value),
            DataValue::Date(value) => serializer.serialize_str(&value.to_string()),
        }
    }
}
