use crossterm::event::{KeyCode, KeyEvent};

#[allow(unused)]
use tracing::info;

use crate::app::{table_data::data_table::EditTarget, App};

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        // let is_editing = self.data.editing.is_some();
        // info!("{:#?}", key_event);
        match self.data.edit_target {
            EditTarget::None => {
                if let Some(action) = self.key_bindings.normal.get(&key_event) {
                    info!("Normal {:#?}", action);
                    self.perform_action(action.clone());
                }
            }
            EditTarget::Cell(_cell) => {
                self.intercept_edits(&key_event);
                if let Some(action) = self.key_bindings.edit.get(&key_event) {
                    info!("Edit {:#?}", action);
                    self.perform_action(action.clone());
                }
            }
            EditTarget::Header(_col) => {
                self.intercept_edits(&key_event);
                if let Some(action) = self.key_bindings.edit.get(&key_event) {
                    info!("Edit {:#?}", action);
                    self.perform_action(action.clone());
                }
            }
            EditTarget::FileName => {
                self.intercept_edits(&key_event);
                if let Some(action) = self.key_bindings.edit.get(&key_event) {
                    info!("Edit {:#?}", action);
                    self.perform_action(action.clone());
                }
            }
        }
    }

    fn intercept_edits(&mut self, key_event: &KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => self.data.insert_char(c),
            KeyCode::Backspace => self.data.delete_backwards(),
            KeyCode::Delete => self.data.delete_forwards(),

            _ => {}
        }
    }
}
