use std::path::PathBuf;

use crate::backend::{
    file_formats::{file_csv::CsvDescription, file_multi::FileDescription},
    tasks::events::IoCommand,
};

use super::DataTable;

impl DataTable {
    pub fn save_csv_command(&self) -> IoCommand {
        IoCommand::SaveCsv(CsvDescription {
            df: self.df.clone(),
            delim: self.delim,
            errors: vec![],
            path: self.path.clone(),
        })
    }

    pub fn save_json_command(&self) -> IoCommand {
        let rows = self.df.mapped_rows_owned();
        let path = match self.path {
            Some(ref path) => {
                let mut path = path.clone();
                path.set_extension("json");
                path
            }
            None => PathBuf::from("export.json"),
        };
        IoCommand::SaveJson(FileDescription { path, rows })
    }

    pub fn save_yml_command(&self) -> IoCommand {
        let rows = self.df.mapped_rows_owned();
        let path = match self.path {
            Some(ref path) => {
                let mut path = path.clone();
                path.set_extension("yml");
                path
            }
            None => PathBuf::from("export.yml"),
        };
        IoCommand::SaveYml(FileDescription { path, rows })
    }

    pub fn save_ron_command(&self) -> IoCommand {
        let rows = self.df.mapped_rows_owned();
        let path = match self.path {
            Some(ref path) => {
                let mut path = path.clone();
                path.set_extension("ron");
                path
            }
            None => PathBuf::from("export.ron"),
        };
        IoCommand::SaveRon(FileDescription { path, rows })
    }

    pub fn save_toml_command(&self) -> IoCommand {
        let rows = self.df.mapped_rows_owned();
        let path = match self.path {
            Some(ref path) => {
                let mut path = path.clone();
                path.set_extension("toml");
                path
            }
            None => PathBuf::from("export.toml"),
        };
        IoCommand::SaveToml(FileDescription { rows, path })
    }
}
