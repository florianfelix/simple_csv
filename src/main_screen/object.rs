// #![feature(arbitrary_self_types_pointers)]
// #![feature(arbitrary_self_types)]
#![allow(unused)]
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum FieldError {
    Parse(std::num::ParseFloatError),
}

impl From<std::num::ParseFloatError> for FieldError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::Parse(value)
    }
}

impl std::error::Error for FieldError {}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Fields(pub Vec<Box<dyn FieldTrait>>);

impl Fields {
    pub fn to_value(&self) -> serde_json::Value {
        let v = serde_json::Value::String(String::from("value"));

        let arr = self
            .0
            .iter()
            .map(|f| serde_json::Value::String(f.name()))
            .collect_vec();

        serde_json::Value::Array(arr)
    }
    pub fn to_json(&self) -> String {
        let v = self.to_value();
        serde_json::to_string(&v).unwrap()
    }
}

impl Default for Fields {
    fn default() -> Self {
        Self(vec![
            Box::new(StringField {
                name: String::from("Name"),
                value: String::from("default Name"),
            }),
            Box::new(StringField {
                name: String::from("Comment"),
                value: String::from("Some glorious comment!"),
            }),
            Box::new(FloatField {
                name: String::from("Amount"),
                value: 0.0,
            }),
        ])
    }
}

// TRAIT
pub trait FieldTrait: std::fmt::Debug {
    fn name(&self) -> String;
    fn print(&self) -> String;
    fn parse(&mut self, value: &str) -> Result<(), FieldError>;
}

#[derive(Default, Debug, Clone)]
pub struct StringField {
    name: String,
    value: String,
}

impl FieldTrait for StringField {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn print(&self) -> String {
        self.value.clone()
    }
    fn parse(&mut self, value: &str) -> Result<(), FieldError> {
        self.value = String::from(value);
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
pub struct FloatField {
    name: String,
    value: f32,
}

impl FieldTrait for FloatField {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn print(&self) -> String {
        format!("{:}", self.value)
    }
    fn parse(&mut self, value: &str) -> Result<(), FieldError> {
        self.value = value.parse::<f32>()?;
        Ok(())
    }
}

// fn test() {
//     let mut ss = StringField {
//         name: String::from("Name"),
//         value: String::from(""),
//     };
//     let mut fs = FloatField {
//         name: String::from("Name"),
//         value: 0.0,
//     };
//     let _s = print_it(&mut ss);

//     let f = Fields(vec![Box::new(ss), Box::new(fs)]);
//     let names = f.0.iter().map(|f| f.name()).collect::<Vec<_>>();
// }

// fn print_it(v: &mut dyn FieldTrait) -> String {
//     v.parse("new value").unwrap();
//     v.print()
// }

// #[derive(Default, Debug, Clone)]
// pub struct Object(Vec<Field>);

// impl Field {
//     fn print(&self) -> AppResult<String> {
//         Ok(serde_json::from_value(self.value.clone())?)
//     }
// }
