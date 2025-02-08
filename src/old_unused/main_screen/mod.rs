use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{layout::Rect, text::Line, Frame};
use table_data::data_table::DataTable;

#[allow(unused)]
use tracing::info;

use crate::{
    app::App,
    event::{csv::CsvDescription, IoCommandError, IoCommandResult},
    utils::layout_helpers::triple_pane_percantages,
};

pub mod table_data;

#[derive(Debug)]
pub struct MainScreen {
    pub name: String,
    pub data_table: DataTable,
    pub action_error: Option<IoCommandError>,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            name: String::from("Main Screen"),
            data_table: DataTable::default(),
            action_error: None,
        }
    }
}

impl MainScreen {
    pub fn from_parsed_csv(&mut self, data: IoCommandResult<CsvDescription>) {
        match data {
            Ok(csv) => {
                self.action_error = None;
                self.data_table = DataTable::default()
                    .set_headers(csv.data.headers)
                    .set_rows(csv.data.rows)
                    .set_parse_errors(csv.errors)
                    .set_path(csv.path)
                    .set_delim(csv.delim);
            }
            Err(e) => {
                // panic!("{}", e);
                self.action_error = Some(e);
                self.data_table = DataTable::default();
            }
        }
    }
}
impl MainScreen {
    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        let [_left, _center, _right] = triple_pane_percantages(20, 40, 40, area);

        if self.data_table.width() > 0 {
            self.data_table.render(frame, area);
        } else if let Some(e) = &self.action_error {
            let txt = Line::from(e.to_string());
            frame.render_widget(txt, area);
        }
    }

    pub fn key_handler(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        let is_editing = app.main_screen.data_table.editing.is_some();

        if is_editing {
            Self::key_consumer_edit(key_event, app)
        } else {
            Self::key_consumer_normal(key_event, app)
        }
    }

    fn key_consumer_normal(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        match key_event.code {
            // KeyCode::Down => app.main_screen.data_table.table_state.select_next(),
            // KeyCode::Up => app.main_screen.data_table.table_state.select_previous(),
            KeyCode::Down => app.main_screen.data_table.select_cell_down(),
            KeyCode::Up => app.main_screen.data_table.select_cell_up(),
            KeyCode::Right => app.main_screen.data_table.select_cell_right(),
            KeyCode::Left => app.main_screen.data_table.select_cell_left(),
            KeyCode::Enter => app.main_screen.data_table.toggle_edit(),
            KeyCode::PageUp => app.main_screen.data_table.table_state.select_first(),
            KeyCode::PageDown => app.main_screen.data_table.table_state.select_last(),
            KeyCode::Char(' ') => app.main_screen.data_table.append_row(),
            KeyCode::Char('s') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.io_command_sender
                        .send(app.main_screen.data_table.action_save())
                        .expect("IoCommand Receiver Closed. Quitting");
                }
            }
            _ => return Some(key_event),
        }
        None
    }

    fn key_consumer_edit(key_event: KeyEvent, app: &mut App) -> Option<KeyEvent> {
        let buffer = &mut app.main_screen.data_table.buffer;

        match key_event.code {
            KeyCode::Enter => app.main_screen.data_table.toggle_edit(),
            KeyCode::Tab => {
                app.main_screen.data_table.toggle_edit();
                app.main_screen.data_table.select_cell_right();
            }
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Up => {
                app.main_screen.data_table.toggle_edit();
                app.main_screen.data_table.table_state.select_previous();
            }
            KeyCode::Down => {
                app.main_screen.data_table.toggle_edit();
                app.main_screen.data_table.table_state.select_next();
            }
            KeyCode::Right => {
                app.main_screen.data_table.toggle_edit();
                app.main_screen.data_table.select_cell_right();
            }
            KeyCode::Left => {
                app.main_screen.data_table.toggle_edit();
                app.main_screen.data_table.select_cell_left();
            }

            _ => return Some(key_event),
        }
        None
    }
}
