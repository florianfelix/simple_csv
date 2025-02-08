use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[allow(unused)]
use tracing::info;

use crate::{app::App, AppResult};

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> AppResult<()> {
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

        Ok(())
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
    // Handles the key events and updates the state of [`App`].
    pub fn handle_key_events1(&mut self, key_event: KeyEvent) -> AppResult<()> {
        // info!("{:#?}", key_event);
        let mut maybe_remaining_event = None;

        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            maybe_remaining_event = self.key_handler(key_event);
        }

        if let Some(key_event) = maybe_remaining_event {
            self.base_key_events(key_event)?;
        }

        Ok(())
    }

    pub fn base_key_events(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                self.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.quit();
                }
            }
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    pub fn key_handler(&mut self, key_event: KeyEvent) -> Option<KeyEvent> {
        let is_editing = self.data.editing.is_some();

        if is_editing {
            self.key_consumer_edit(key_event)
        } else {
            self.key_consumer_normal(key_event)
        }
    }

    fn key_consumer_normal(&mut self, key_event: KeyEvent) -> Option<KeyEvent> {
        match key_event.code {
            // KeyCode::Down => app.main_screen.data_table.table_state.select_next(),
            // KeyCode::Up => app.main_screen.data_table.table_state.select_previous(),
            KeyCode::Down => self.data.select_cell_down(),
            KeyCode::Up => self.data.select_cell_up(),
            KeyCode::Right => self.data.select_cell_right(),
            KeyCode::Left => self.data.select_cell_left(),
            KeyCode::Enter => self.data.toggle_edit(),
            KeyCode::PageUp => self.data.table_state.select_first(),
            KeyCode::PageDown => self.data.table_state.select_last(),
            KeyCode::Char(' ') => self.data.append_row(),
            KeyCode::Char('s') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    self.save();
                }
            }
            KeyCode::Char('k') => self.reload_key_bindings(),
            _ => return Some(key_event),
        }
        None
    }

    fn key_consumer_edit(&mut self, key_event: KeyEvent) -> Option<KeyEvent> {
        let buffer = &mut self.data.buffer;

        match key_event.code {
            KeyCode::Enter => self.data.toggle_edit(),
            KeyCode::Tab => {
                self.data.toggle_edit();
                self.data.select_cell_right();
            }
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Up => {
                self.data.toggle_edit();
                self.data.table_state.select_previous();
            }
            KeyCode::Down => {
                self.data.toggle_edit();
                self.data.table_state.select_next();
            }
            KeyCode::Right => {
                self.data.toggle_edit();
                self.data.select_cell_right();
            }
            KeyCode::Left => {
                self.data.toggle_edit();
                self.data.select_cell_left();
            }

            _ => return Some(key_event),
        }
        None
    }
}
