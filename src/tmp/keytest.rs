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
    fn parse(raw: &str) -> KeyEvent;
    fn as_config_string(&self) -> String;
}

impl KeyEventExt for KeyEvent {
    fn parse(raw: &str) -> KeyEvent {
        super::parse::parse_key_event(raw).unwrap()
    }
    fn as_config_string(&self) -> String {
        super::parse::key_event_to_string(self)
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
        let normal: IndexMap<KeyEvent, Action> = IndexMap::from_iter(
            map.get("normal")
                .unwrap()
                .iter()
                .map(|(k, v)| (KeyEvent::parse(k), v.clone()))
                .collect_vec(),
        );
        let edit: IndexMap<KeyEvent, Action> = IndexMap::from_iter(
            map.get("edit")
                .unwrap()
                .iter()
                .map(|(k, v)| (KeyEvent::parse(k), v.clone()))
                .collect_vec(),
        );
        Self { normal, edit }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveMaps {
    pub normal: IndexMap<String, Action>,
    pub edit: IndexMap<String, Action>,
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

fn default_keymap() -> IndexMap<KeyEvent, Action> {
    // let map = IndexMap::new();
    let mut map_normal: IndexMap<KeyEvent, Action> = IndexMap::new();

    let key = KeyEvent::new(KeyCode::Up, KeyModifiers::CONTROL);
    map_normal.entry(key).insert_entry(Action::ToggleEdit);

    let key = KeyEvent::new(
        KeyCode::Char('s'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    );

    map_normal.entry(key).insert_entry(Action::Save);

    map_normal
}

fn to_saveable() -> IndexMap<String, Action> {
    let input = default_keymap();

    let mut save: IndexMap<String, Action> = IndexMap::new();
    input.iter().for_each(|(key, value)| {
        save.insert(super::parse::key_event_to_string(key), value.clone());
    });
    save
}

fn from_savable(input: SaveMaps) -> KeyMaps {
    let mut key_map = KeyMaps {
        normal: IndexMap::new(),
        edit: IndexMap::new(),
    };
    // let mut normal = IndexMap::new()
    for (k, v) in input.normal.iter() {
        let key = super::parse::parse_key_event(k).unwrap();
        key_map.normal.entry(key).insert_entry(v.clone());
    }
    key_map
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
    let savemaps = KeyMaps {
        normal: default_keymap(),
        edit: IndexMap::new(),
    }
    .to_config_map();

    serialize_ron(&savemaps).await;
    serialize_toml(&savemaps).await;
    serialize_json5(&savemaps).await;
    serialize_yml(&savemaps).await;

    let key_map = KeyMaps::from_config_map(savemaps);
    info!("{:#?}", key_map);
}
