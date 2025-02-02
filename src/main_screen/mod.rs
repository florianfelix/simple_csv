use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};
use table_data::data_table::DataTable;

use crate::{app::App, utils::layout_helpers::triple_pane_percantages};

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
    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        let [_left, _center, _right] = triple_pane_percantages(20, 40, 40, area);

        self.data_table.render_table(frame, area);
    }

    pub fn key_handler(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        let is_editing = app.main_screen.data_table.editing.is_some();

        let maybe_event = if is_editing {
            Self::key_consumer_edit(key_event, app)
        } else {
            Self::key_consumer_normal(key_event, app)
        };

        maybe_event
    }

    fn key_consumer_normal(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        match key_event.code {
            KeyCode::Down => app.main_screen.data_table.table_state.select_next(),
            KeyCode::Up => app.main_screen.data_table.table_state.select_previous(),
            KeyCode::Right => app.main_screen.data_table.select_cell_next(),
            KeyCode::Left => app.main_screen.data_table.select_cell_previous(),
            KeyCode::Enter => app.main_screen.data_table.toggle_edit(),
            _ => return Some(key_event),
        }
        // info!("Normal {:#?}", app.main_screen.data_table.editing);
        None
    }

    fn key_consumer_edit(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        // let edit = &mut app.main_screen.name;
        let buffer = &mut app.main_screen.data_table.buffer;

        match key_event.code {
            KeyCode::Enter => app.main_screen.data_table.toggle_edit(),
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => {
                buffer.pop();
            }

            KeyCode::Esc => app.main_screen.mode = Mode::Normal,
            _ => return Some(key_event),
        }
        None
    }
}
