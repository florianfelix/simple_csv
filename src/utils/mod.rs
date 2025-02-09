use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub mod cli;
pub mod logging;

pub fn get_config_path() -> Option<PathBuf> {
    static KEYMAPPATH: OnceLock<Option<PathBuf>> = OnceLock::new();
    KEYMAPPATH.get_or_init(keymap_path).clone()
}

fn keymap_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("de", "hil", "tui_data_entry") {
        let conf_dir = proj_dirs.config_dir().to_path_buf();
        let conf_path = conf_dir.join(Path::new("keymap.yml"));
        let exists = conf_path.try_exists();
        if let Ok(true) = exists {
            return Some(conf_path);
        }
    }
    None
}
