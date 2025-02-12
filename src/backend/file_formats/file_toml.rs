use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TomlDescription {
    pub rows: Vec<IndexMap<String, String>>,
}

impl TomlDescription {
    pub fn to_toml_string(&self) -> AppResult<String> {
        let res = toml::to_string(self)?;
        Ok(res)
    }
}
