use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use indexmap::IndexMap;
use itertools::Itertools;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

#[allow(unused)]
use tracing::info;

use crate::{app::App, event::csv::save_file};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct KeyMaps {
    normal: IndexMap<KeyEvent, Action>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SaveMaps {
    normal: IndexMap<String, Action>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Action {
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

fn key_event_to_string(key: &KeyEvent) -> String {
    let mut ms = key.modifiers.iter().map(|m| m.to_string()).collect_vec();
    // ms.push(key.code.to_string());
    ms.push(format!("{:?}", key.code));
    ms.join(" ")
}

fn key_event_from_string(input: &str) {}

fn to_saveable() -> IndexMap<String, Action> {
    let input = default_keymap();

    let mut save: IndexMap<String, Action> = IndexMap::new();
    input.iter().for_each(|(key, value)| {
        save.insert(key_event_to_string(key), value.clone());
    });
    save
}

pub async fn serialize_toml() {
    let s = SaveMaps {
        normal: to_saveable(),
    };
    let s = toml::to_string(&s).unwrap();
    save_file(&PathBuf::from("keymap.toml"), &s).await.unwrap();
}

pub async fn serialize_json5() {
    let s = SaveMaps {
        normal: to_saveable(),
    };
    let s = json5::to_string(&s).unwrap();
    save_file(&PathBuf::from("keymap.json5"), &s).await.unwrap();
}

pub async fn serialize_yml() {
    let s = SaveMaps {
        normal: to_saveable(),
    };
    let s = serde_yml::to_string(&s).unwrap();
    save_file(&PathBuf::from("keymap.yml"), &s).await.unwrap();
}
pub async fn serialize_ron() {
    let input = default_keymap();

    let m = KeyMaps { normal: input };
    // info!("{:#?}", &input);
    let s = to_string_pretty(
        &m,
        PrettyConfig::new()
            .depth_limit(4)
            .struct_names(true)
            .indentor("  ".to_owned()),
    )
    .unwrap();

    // info!("INDEXMAP \n{:#?}", &s);
    save_file(&PathBuf::from("keymap.ron"), &s).await.unwrap();

    let _v: KeyMaps = ron::from_str(&s).unwrap();
    // info!("{:#?}", &v);
}

pub async fn serialise_test() {
    serialize_ron().await;
    serialize_toml().await;
    serialize_json5().await;
    serialize_yml().await;
}
