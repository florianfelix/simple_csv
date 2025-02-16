use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FileDescription {
    pub path: PathBuf,
    pub rows: Vec<IndexMap<String, String>>,
}

impl FileDescription {
    pub fn to_json_string(&self) -> AppResult<String> {
        let res = serde_json::to_string_pretty(&self.rows)?;
        Ok(res)
    }
    pub fn to_json5_string(&self) -> AppResult<String> {
        let res = json5::to_string(&self.rows)?;
        Ok(res)
    }
    pub fn to_yml_string(&self) -> AppResult<String> {
        let res = serde_yml::to_string(&self.rows)?;
        Ok(res)
    }
    pub fn to_ron_string(&self) -> AppResult<String> {
        let res = ron::ser::to_string_pretty(
            &self.rows,
            ron::ser::PrettyConfig::new()
                .depth_limit(4)
                .indentor("  ".to_string()),
        )?;
        Ok(res)
    }
    pub fn to_toml_string(&self) -> AppResult<String> {
        let res = toml::to_string(self)?;
        Ok(res)
    }
}
