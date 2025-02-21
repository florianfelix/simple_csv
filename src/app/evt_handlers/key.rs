use crossterm::event::{KeyCode, KeyEvent};

#[allow(unused)]
use tracing::info;

use crate::app::{component_table::EditTarget, App};

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        // info!("{:#?}", key_event);
        if let Some(key_combination) = self.combiner.transform(key_event) {
            // info!("{:?}", key_combination);
            match self.data.edit_target {
                EditTarget::None => {
                    if let Some(action) = self.key_bindings.normal.get(&key_combination) {
                        // info!("Normal {:#?}", action);
                        self.perform_action(action.clone());
                    }
                }
                _ => {
                    self.intercept_edits(&key_event);
                    if let Some(action) = self.key_bindings.edit.get(&key_combination) {
                        // info!("Edit {:#?}", action);
                        self.perform_action(action.clone());
                    }
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
