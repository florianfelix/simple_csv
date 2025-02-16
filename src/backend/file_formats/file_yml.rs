use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct YmlDescription {
    pub path: PathBuf,
    pub rows: Vec<IndexMap<String, String>>,
}

impl YmlDescription {
    pub fn to_yml_string(&self) -> AppResult<String> {
        let res = serde_yml::to_string(&self.rows)?;
        Ok(res)
    }
}
