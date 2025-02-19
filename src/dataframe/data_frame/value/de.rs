use serde::{de::Visitor, Deserialize, Deserializer};
use std::fmt;

use super::{DataValue, Float};
// use crate::value::float::Float;

struct ValueVisitor;

impl<'de> Deserialize<'de> for DataValue {
    fn deserialize<D>(deserializer: D) -> Result<DataValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

// impl<'de> Visitor<'de> for ValueVisitor {
impl Visitor<'_> for ValueVisitor {
    type Value = DataValue;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("SOMETHING HERE")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING BOOL {:#?}", v);
        Ok(DataValue::Bool(v))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING U32 {:#?}", v);
        Ok(DataValue::Int(v as i64))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING U64 {:#?}", v);
        Ok(DataValue::Int(v as i64))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING F32 {:#?}", v);
        Ok(DataValue::Float(Float::new(v as f64)))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING F64 {:#?}", v);
        Ok(DataValue::Float(Float::new(v)))
    }
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING NONE");
        Ok(DataValue::Null)
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING UNIT");
        Ok(DataValue::Null)
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // println!("VISITING &str {:#?}", v);
        if v.is_empty() {
            Ok(DataValue::Null)
        } else {
            Ok(DataValue::String(v.to_owned()))
        }
    }
}
