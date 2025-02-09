use crossterm::event::{KeyCode, KeyEvent};

#[allow(unused)]
use tracing::info;

use crate::app::App;

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        let is_editing = self.data.editing.is_some();

        if is_editing {
            self.intercept_edits(key_event);
            if let Some(action) = self.key_bindings.edit.get(&key_event) {
                self.perform_action(action.clone());
            }
        } else if let Some(action) = self.key_bindings.normal.get(&key_event) {
            info!("{:#?}", action);
            self.perform_action(action.clone());
        }
    }

    fn intercept_edits(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => self.data.buffer.push(c),
            KeyCode::Backspace => {
                self.data.buffer.pop();
            }
            _ => {}
        }
    }
}
