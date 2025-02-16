use std::path::PathBuf;

use indexmap::IndexMap;
use itertools::Itertools;

use crate::backend::{
    file_formats::{
        file_csv::{CsvData, CsvDescription},
        file_json::JsonDescription,
        file_ron::RonDescription,
        file_toml::TomlDescription,
        file_yml::YmlDescription,
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

    pub fn save_json_command(&self) -> IoCommand {
        let rows = self
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
                path.set_extension("json");
                path
            }
            None => PathBuf::from("export.json"),
        };
        IoCommand::SaveJson(JsonDescription { path, rows })
    }

    pub fn save_yml_command(&self) -> IoCommand {
        let rows = self
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
                path.set_extension("yml");
                path
            }
            None => PathBuf::from("export.yml"),
        };
        IoCommand::SaveYml(YmlDescription { path, rows })
    }

    pub fn save_ron_command(&self) -> IoCommand {
        let rows = self
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
                path.set_extension("ron");
                path
            }
            None => PathBuf::from("export.ron"),
        };
        IoCommand::SaveRon(RonDescription { path, rows })
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
