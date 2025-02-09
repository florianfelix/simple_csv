#[allow(unused)]
use tracing::info;

use crate::{app::App, backend::tasks::events::IoEvent};

impl App {
    pub fn handle_io_events(&mut self, io_event: IoEvent) {
        // info!("{:#?}", io_event);
        use IoEvent::*;
        match io_event {
            LoadedCsv(parsed) => self.from_parsed_csv(parsed),
            SavedCsv => self.data.is_dirty = false,
            LoadedKeybindings(key_bindings) => self.set_key_bindings(key_bindings),
        }
    }
}
