#![allow(unused)]
// use notify_debouncer_full::{new_debouncer, notify, DebouncedEvent, Debouncer, NoCache};
// use serde::{Deserialize, Serialize};

use crate::AppResult;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    value: bool,
    num: Option<i32>,
}

impl Settings {
    const PATH: &str = ".config/settings.toml";

    pub fn get() -> &'static std::sync::RwLock<Settings> {
        static CONFIG: std::sync::OnceLock<std::sync::RwLock<Settings>> =
            std::sync::OnceLock::new();
        CONFIG.get_or_init(|| match Self::load() {
            Ok(settings) => std::sync::RwLock::new(settings),
            Err(_e) => {
                let settings = Settings::default();
                Self::save(&settings).unwrap();
                std::sync::RwLock::new(Settings::default())
            }
        })
    }
    fn load() -> AppResult<Settings> {
        let path = std::path::Path::new(Self::PATH);
        let content = std::fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    fn save(settings: &Settings) -> AppResult<()> {
        let path = std::path::Path::new(Self::PATH);

        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        let contents = toml::to_string(settings)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    fn save_current() -> AppResult<()> {
        let path = std::path::Path::new(Self::PATH);

        let settings = Self::get().read().unwrap();

        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        let contents = toml::to_string(&*settings)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    fn refresh() -> AppResult<()> {
        let mut s = Self::get().write().map_err(|_e| {
            // color_eyre::Report::new(std::sync::PoisonError::new("Failed to aquire lock"))
            String::from("Failed to aquire lock")
        })?;
        *s = Self::load()?;
        Ok(())
    }

    pub fn watch() -> AppResult<
        notify_debouncer_full::Debouncer<
            notify_debouncer_full::notify::INotifyWatcher,
            notify_debouncer_full::NoCache,
        >,
    > {
        let mut debouncer = notify_debouncer_full::new_debouncer(
            std::time::Duration::from_secs_f32(0.5),
            None,
            Self::handle_event,
        )?;
        let _ = Self::get();
        debouncer.watch(
            Self::PATH,
            notify_debouncer_full::notify::RecursiveMode::Recursive,
        )?;
        Ok(debouncer)
    }

    fn handle_event(event: notify_debouncer_full::DebounceEventResult) {
        match event {
            Ok(events) => events.iter().for_each(Settings::handle_debounced),
            Err(_errors) => {}
        }
    }

    fn handle_debounced(event: &notify_debouncer_full::DebouncedEvent) {
        #[allow(clippy::single_match)]
        match event {
            notify_debouncer_full::DebouncedEvent {
                event:
                    notify_debouncer_full::notify::Event {
                        kind: notify_debouncer_full::notify::EventKind::Modify(_),
                        ..
                    },
                ..
            } => {
                Settings::refresh().unwrap();
                println!("EventHandler \n{:#?}", Settings::get().write());
            }
            _ => {}
        }
    }
}

// struct Bounce;
// impl DebounceEventHandler for Bounce {
//     fn handle_event(&mut self, event: notify_debouncer_full::DebounceEventResult) {
//         match event {
//             Ok(events) => events.iter().for_each(|event| Settings::handle_evt(event)),
//             Err(_errors) => {}
//         }
//     }
// }
