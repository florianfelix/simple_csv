use std::path::PathBuf;

use indexmap::IndexMap;
use itertools::Itertools;

use crate::backend::{
    file_formats::{
        file_csv::{CsvData, CsvDescription},
        file_toml::TomlDescription,
    },
    tasks::events::IoCommand,
};

use super::DataTable;

impl DataTable {
    pub fn save_csv_command(&self) -> IoCommand {
        let data = CsvData {
            headers: self.headers.clone(),
            rows: self.rows.clone(),
        };
        IoCommand::SaveCsv(CsvDescription {
            data,
            delim: self.delim,
            errors: vec![],
            path: self.path.clone(),
        })
    }

    pub fn save_toml_command(&self) -> IoCommand {
        let data = self
            .rows
            .iter()
            .map(|row| {
                let mut map = IndexMap::new();
                row.iter().zip(self.headers.clone()).for_each(|(v, k)| {
                    map.insert(k, v.to_owned());
                });
                map
            })
            .collect_vec();
        let path = match self.path {
            Some(ref path) => {
                let mut path = path.clone();
                path.set_extension("toml");
                path
            }
            None => PathBuf::from("export.toml"),
        };
        IoCommand::SaveToml(TomlDescription { rows: data, path })
    }
}
