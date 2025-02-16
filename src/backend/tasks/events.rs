use std::path::PathBuf;

use crossterm::event::{KeyEvent, MouseEvent};

use crate::backend::{
    file_formats::{file_csv::CsvDescription, file_multi::FileDescription},
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
    SaveToml(FileDescription),
    SaveJson(FileDescription),
    SaveYml(FileDescription),
    SaveRon(FileDescription),
}

#[derive(Clone, Debug)]
pub enum IoEvent {
    LoadedCsv(IoCommandResult<CsvDescription>),
    SavedCsv,
    LoadedKeybindings(IoCommandResult<KeyBindings>),
}
