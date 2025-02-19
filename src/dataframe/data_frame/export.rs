use indexmap::IndexMap;
use itertools::Itertools;

use super::{DataFrame, DataValue};

impl DataFrame {
    pub fn mapped_rows(&self) -> Vec<IndexMap<&str, &DataValue>> {
        self.rows
            .iter()
            .map(|row| {
                let mut map = IndexMap::new();
                row.iter()
                    .zip(self.headers.iter())
                    .for_each(|(value, header)| {
                        map.insert(header.name(), value);
                    });
                map
            })
            .collect_vec()
    }

    pub fn mapped_rows_owned(&self) -> Vec<IndexMap<String, DataValue>> {
        self.rows
            .iter()
            .map(|row| {
                let mut map = IndexMap::new();
                row.iter()
                    .zip(self.headers.iter())
                    .for_each(|(value, header)| {
                        map.insert(header.name().to_string(), value.clone());
                    });
                map
            })
            .collect_vec()
    }

    pub fn to_ron(&self) -> String {
        ron::ser::to_string_pretty(
            &self.mapped_rows(),
            ron::ser::PrettyConfig::new()
                .depth_limit(4)
                .indentor("  ".to_string()),
        )
        .unwrap()
    }
}
