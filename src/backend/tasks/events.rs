use std::path::PathBuf;

use crossterm::event::{KeyEvent, MouseEvent};

use crate::backend::{key_bindings::KeyBindings, CsvDescription, IoCommandResult};

#[derive(Clone, Debug)]
pub enum BackendEvent {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    ParsedCsv(IoCommandResult<CsvDescription>),
    LoadedKeybindings(IoCommandResult<KeyBindings>),
}

#[derive(Clone, Debug)]
pub enum IoCommand {
    SaveCsv(CsvDescription),
    LoadCsv { path: PathBuf, delim: char },
    LoadKeyBindings,
}
