use std::path::PathBuf;

use crossterm::event::{KeyEvent, MouseEvent};

use crate::backend::{
    file_formats::{
        file_csv::CsvDescription, file_json::JsonDescription, file_toml::TomlDescription,
        file_yml::YmlDescription,
    },
    key_bindings::KeyBindings,
    IoCommandResult,
};

#[derive(Clone, Debug)]
pub enum BackendEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    IoEvent(IoEvent),
}

#[derive(Clone, Debug)]
pub enum IoCommand {
    SaveCsv(CsvDescription),
    LoadCsv { path: PathBuf, delim: char },
    LoadKeyBindings,
    SaveKeyBindings(KeyBindings),
    SaveToml(TomlDescription),
    SaveJson(JsonDescription),
    SaveYml(YmlDescription),
}

#[derive(Clone, Debug)]
pub enum IoEvent {
    LoadedCsv(IoCommandResult<CsvDescription>),
    SavedCsv,
    LoadedKeybindings(IoCommandResult<KeyBindings>),
}
