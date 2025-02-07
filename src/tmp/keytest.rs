use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;
use itertools::Itertools;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

#[allow(unused)]
use tracing::info;

use crate::{app::App, event::csv::save_file};

pub trait KeyEventExt {
    fn parse(raw: &str) -> Result<KeyEvent, String>;
    fn as_config_string(&self) -> String;
}

impl KeyEventExt for KeyEvent {
    fn parse(raw: &str) -> Result<KeyEvent, String> {
        super::parse::parse_key_event(raw)
    }
    fn as_config_string(&self) -> String {
        super::parse::key_event_to_string(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    ToggleEdit,
    Save,
}

#[allow(unused)]
impl Action {
    pub fn act(&self, app: &mut App) {
        match self {
            Self::ToggleEdit => {
                app.data.toggle_edit();
            }
            Self::Save => {
                app.data.action_save();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyMaps {
    pub normal: IndexMap<KeyEvent, Action>,
    pub edit: IndexMap<KeyEvent, Action>,
}
impl KeyMaps {
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

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            normal: default_keymap_normal(),
            edit: default_keymap_edit(),
        }
    }
}

fn default_keymap_normal() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [
        (
            KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
            Action::ToggleEdit,
        ),
        (
            KeyEvent::new(
                KeyCode::Char('s'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
            Action::Save,
        ),
    ];
    IndexMap::from_iter(map)
}
fn default_keymap_edit() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let map = [(
        KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()),
        Action::ToggleEdit,
    )];
    IndexMap::from_iter(map)
}

pub async fn serialize_toml(savemaps: &impl Serialize) {
    let s = toml::to_string(savemaps).unwrap();
    save_file(&PathBuf::from("keymap.toml"), &s).await.unwrap();
}

pub async fn serialize_json5(savemaps: &impl Serialize) {
    let s = json5::to_string(savemaps).unwrap();
    save_file(&PathBuf::from("keymap.json5"), &s).await.unwrap();
}

pub async fn serialize_yml(savemaps: &impl Serialize) {
    let s = serde_yml::to_string(savemaps).unwrap();
    save_file(&PathBuf::from("keymap.yml"), &s).await.unwrap();
}

pub async fn serialize_ron(savemaps: &impl Serialize) {
    let s = to_string_pretty(
        savemaps,
        PrettyConfig::new()
            .depth_limit(4)
            .struct_names(true)
            .indentor("  ".to_owned()),
    )
    .unwrap();
    save_file(&PathBuf::from("keymap.ron"), &s).await.unwrap();
}

pub async fn serialise_test() {
    let savemaps = KeyMaps::default().to_config_map();

    serialize_ron(&savemaps).await;
    serialize_toml(&savemaps).await;
    serialize_json5(&savemaps).await;
    serialize_yml(&savemaps).await;

    let key_map = KeyMaps::from_config_map(savemaps);
    info!("{:#?}", key_map);
}
