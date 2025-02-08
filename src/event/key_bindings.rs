use map::KeyBindings;
use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::fs;

use tracing::error;
#[allow(unused)]
use tracing::info;

use super::IoTaskResult;

mod action;
mod defaults;
mod map;
mod utils;

pub struct KeyBindingsIo;

impl KeyBindingsIo {
    const KEYMAP: &str = "keymap.yml";
    pub async fn load() -> IoTaskResult<KeyBindings> {
        let conf_path = Self::get_conf_path().await;
        info!("LOADING KEYBINDINGS\n{:#?}", conf_path);
        if let Err(e) = conf_path {
            error!("{}", e.to_string());
            return Ok(KeyBindings::default());
        }
        let conf_path = conf_path.unwrap();
        // if file not exists create with defaults and return defaults
        if let Ok(false) = fs::try_exists(&conf_path).await {
            KeyBindings::default().save(&conf_path).await?;
            Ok(KeyBindings::default())
        } else {
            let key_bindings = KeyBindings::load(&conf_path).await?;
            Ok(key_bindings)
        }
    }

    async fn get_conf_path() -> IoTaskResult<PathBuf> {
        if let Ok(current_dir) = env::current_dir() {
            let local = current_dir.join(Path::new(Self::KEYMAP));
            if let Ok(true) = fs::try_exists(&local).await {
                return Ok(local);
            }
        }

        if let Some(proj_dirs) = directories::ProjectDirs::from("de", "hil", "tui_data_entry") {
            // info!("{:#?}", proj_dirs);
            let conf_dir = proj_dirs.config_dir().to_path_buf();
            fs::create_dir_all(&conf_dir).await?;
            let conf_path = conf_dir.join(Path::new(Self::KEYMAP));
            return Ok(conf_path);
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "unable to use either local or user config location",
        )
        .into())
    }
}

// pub async fn serialize_yml(savemaps: &impl Serialize) {
//     let s = serde_yml::to_string(savemaps).unwrap();
//     save_file(&PathBuf::from("keymap.yml"), &s).await.unwrap();
// }
