mod action;
mod io;
mod key;
pub use action::Action;

use crate::backend::tasks::events::BackendEvent;

use super::App;

impl App {
    pub fn handle_backend_events(&mut self, backend_event: BackendEvent) {
        match backend_event {
            BackendEvent::Tick => self.tick(),
            BackendEvent::Key(key_event) => self.handle_key_events(key_event),
            BackendEvent::Mouse(_) => {}
            BackendEvent::Resize(_, _) => {}
            BackendEvent::IoEvent(io_event) => self.handle_io_events(io_event),
        }
    }
}
