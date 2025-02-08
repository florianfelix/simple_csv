use std::path::PathBuf;

use crossterm::event::KeyEvent;
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::event::{
    csv::{path_to_string, save_file},
    IoTaskResult,
};

use super::{
    action::Action,
    defaults::{default_keymap_edit, default_keymap_normal},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyBindings {
    pub normal: IndexMap<KeyEvent, Action>,
    pub edit: IndexMap<KeyEvent, Action>,
}
impl KeyBindings {
    pub async fn save(&self, path: &PathBuf) -> IoTaskResult<()> {
        let map = self.to_config_map();
        let map = serde_yml::to_string(&map).unwrap();
        save_file(path, &map).await?;
        info!("Saved default key bindings: {:#?}", path);
        Ok(())
    }
    pub async fn load(path: &PathBuf) -> IoTaskResult<Self> {
        let text = path_to_string(path).await?;
        let maps: IndexMap<String, IndexMap<String, Action>> = serde_yml::from_str(&text)?;
        let key_bindings = KeyBindings::from_config_map(maps);
        info!("Lodaed key bindings from: {:#?}", path);
        Ok(key_bindings)
    }

    pub fn to_config_map(&self) -> IndexMap<String, IndexMap<String, Action>> {
        let normal: IndexMap<String, Action> = IndexMap::from_iter(
            self.normal
                .iter()
                .map(|(k, v)| (k.as_config_string(), v.clone()))
                .collect_vec(),
        );
        let edit: IndexMap<String, Action> = IndexMap::from_iter(
            self.edit
                .iter()
                .map(|(k, v)| (k.as_config_string(), v.clone()))
                .collect_vec(),
        );
        IndexMap::from_iter([("normal".to_string(), normal), ("edit".to_string(), edit)])
    }
    pub fn from_config_map(map: IndexMap<String, IndexMap<String, Action>>) -> Self {
        let normal = match map.get("normal") {
            Some(normal) => normal.to_owned(),
            None => IndexMap::new(),
        };
        let normal = normal
            .iter()
            .map(|(k, v)| (KeyEvent::parse(k), v.clone()))
            .filter(|(k, _)| k.is_ok())
            .map(|(k, v)| (k.unwrap(), v))
            .collect_vec();
        let normal: IndexMap<KeyEvent, Action> = IndexMap::from_iter(normal);

        let edit = match map.get("edit") {
            Some(edit) => edit.to_owned(),
            None => IndexMap::new(),
        };
        let edit = edit
            .iter()
            .map(|(k, v)| (KeyEvent::parse(k), v.clone()))
            .filter(|(k, _)| k.is_ok())
            .map(|(k, v)| (k.unwrap(), v))
            .collect_vec();
        let edit: IndexMap<KeyEvent, Action> = IndexMap::from_iter(edit);

        Self { normal, edit }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            normal: default_keymap_normal(),
            edit: default_keymap_edit(),
        }
    }
}

pub trait KeyEventExt {
    fn parse(raw: &str) -> Result<KeyEvent, String>;
    fn as_config_string(&self) -> String;
}

impl KeyEventExt for KeyEvent {
    fn parse(raw: &str) -> Result<KeyEvent, String> {
        super::utils::parse_key_event(raw)
    }
    fn as_config_string(&self) -> String {
        super::utils::key_event_to_string(self)
    }
}
