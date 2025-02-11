mod map;

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub use map::KeyBindings;

pub fn keymap_file() -> Option<PathBuf> {
    static KEYMAPPATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    KEYMAPPATH.get_or_init(maybe_existing_keymap_file).clone()
}

fn maybe_existing_keymap_file() -> Option<PathBuf> {
    if let Some(conf_path) = keymap_path() {
        if let Ok(true) = conf_path.try_exists() {
            return Some(conf_path);
        }
    }
    None
}

fn keymap_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("de", "hil", "tui_data_entry") {
        let conf_dir = proj_dirs.config_dir().to_path_buf();
        let conf_path = conf_dir.join(Path::new("keymap.yml"));
        return Some(conf_path);
    }
    None
}
