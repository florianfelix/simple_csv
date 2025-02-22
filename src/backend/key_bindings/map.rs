use crokey::{KeyCombination, KeyCombinationFormat};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[allow(unused)]
use tracing::info;

use crate::{
    app::evt_handlers::Action,
    backend::{
        utils::{read_file, save_file},
        IoCommandError, IoCommandResult,
    },
};

use super::{keymap_file, keymap_path};

const DEFAULTS: &str = include_str!("../../../default_keybindings.yml");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyBindings {
    pub normal: IndexMap<KeyCombination, Action>,
    pub edit: IndexMap<KeyCombination, Action>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        serde_yml::from_str(DEFAULTS).unwrap()
    }
}

impl KeyBindings {
    pub async fn load() -> IoCommandResult<Self> {
        match keymap_file() {
            None => Err(IoCommandError::Io(String::from(
                "Keymap config file not found",
            ))),
            Some(path) => {
                let text = read_file(&path).await?;
                let key_bindings: KeyBindings = serde_yml::from_str(&text)?;
                // info!("\nLoaded key bindings from: {:#?}", path);
                Ok(key_bindings)
            }
        }
    }

    pub async fn save(&self) -> IoCommandResult<()> {
        match keymap_path() {
            None => return Err(IoCommandError::Io(String::from("unable to determine path"))),
            Some(path) => {
                let map = serde_yml::to_string(&self).unwrap();
                save_file(&path, &map).await?;
                // info!("Saved default key bindings: {:#?}", path);
            }
        }
        Ok(())
    }

    pub fn display(&self) -> (Vec<[String; 2]>, Vec<[String; 2]>) {
        let format = KeyCombinationFormat::default();
        let normal = self
            .normal
            .iter()
            .map(|(k, v)| [v.to_string(), format.to_string(k.to_owned())])
            .collect_vec();
        let edit = self
            .edit
            .iter()
            .map(|(k, v)| [v.to_string(), format.to_string(k.to_owned())])
            .collect_vec();
        (normal, edit)
    }
}
