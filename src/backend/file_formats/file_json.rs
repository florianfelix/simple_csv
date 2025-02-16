use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JsonDescription {
    pub path: PathBuf,
    pub rows: Vec<IndexMap<String, String>>,
}

impl JsonDescription {
    pub fn to_json_string(&self) -> AppResult<String> {
        let res = serde_json::to_string_pretty(&self.rows)?;
        Ok(res)
    }
    pub fn to_json5_string(&self) -> AppResult<String> {
        let res = json5::to_string(&self.rows)?;
        Ok(res)
    }
}
