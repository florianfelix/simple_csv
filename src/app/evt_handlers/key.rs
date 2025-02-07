use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::App, main_screen::MainScreen, AppResult};

impl App {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> AppResult<()> {
        let mut maybe_remaining_event = None;

        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            maybe_remaining_event = MainScreen::key_handler(key_event, self);
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
}
