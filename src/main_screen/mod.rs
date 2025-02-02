use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};
use table_data::data_table::DataTable;

use crate::{
    app::App, handler::base_key_events, utils::layout_helpers::triple_pane_percantages, AppResult,
};

pub mod table_data;

#[derive(Debug)]
pub enum TaField {
    Name,
    Amount,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct MainScreen {
    pub mode: Mode,
    pub buffer: String,
    pub name: String,
    pub data_table: DataTable,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            buffer: String::from("Buffer"),
            name: String::from("Main Screen"),
            data_table: DataTable::example(),
        }
    }
}

impl MainScreen {
    pub fn render_body(&self, frame: &mut Frame, area: Rect) {
        let [_left, _center, _right] = triple_pane_percantages(20, 40, 40, area);

        self.data_table.render_table(frame, area);
    }

    pub fn key_handler_edit(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            let edit = &mut app.main_screen.buffer;

            match key_event.code {
                KeyCode::Char(c) => edit.push(c),
                KeyCode::Backspace => {
                    edit.pop();
                }

                KeyCode::Esc => app.main_screen.mode = Mode::Normal,
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }

    pub fn key_handler_normal(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            // let edit = &mut app.main_screen.name;

            match key_event.code {
                KeyCode::Char('e') => app.main_screen.mode = Mode::Editing,
                // KeyCode::Char('j') => info!("\n{:#?}", app.main_screen.fields.to_json()),
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }
}
