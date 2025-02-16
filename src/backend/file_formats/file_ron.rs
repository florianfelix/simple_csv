use std::path::PathBuf;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RonDescription {
    pub path: PathBuf,
    pub rows: Vec<IndexMap<String, String>>,
}

impl RonDescription {
    pub fn to_ron_string(&self) -> AppResult<String> {
        let res = ron::ser::to_string_pretty(
            &self.rows,
            ron::ser::PrettyConfig::new()
                .depth_limit(4)
                .indentor("  ".to_string()),
        )?;
        Ok(res)
    }
}
