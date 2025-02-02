use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
// use object::Fields;
use ratatui::{
    layout::{Constraint, Rect},
    style::Style,
    widgets::{Block, Borders, Table},
    Frame,
};
use table_data::data_row::DataRow;

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
    pub data_rows: Vec<DataRow>,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            buffer: String::from("Buffer"),
            name: String::from("Main Screen"),
            data_rows: DataRow::examples(),
        }
    }
}

impl MainScreen {
    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        let [_left, _center, _right] = triple_pane_percantages(20, 40, 40, area);

        self.render_table(frame, area);
    }
    fn render_table(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Table");

        let widths = self.equal_row_widths();
        let rows = self.data_rows.iter().map(|r| r.rat_row()).collect_vec();
        let table = Table::new(rows, widths).block(block);

        frame.render_widget(table, area);
    }
    fn equal_row_widths(&self) -> Vec<Constraint> {
        if !self.data_rows.is_empty() {
            let n = self.data_rows.first().unwrap().headers().len();
            let equal: u16 = (100 / n) as u16;
            let mut v = vec![];
            for _ in 0..n {
                v.push(Constraint::Percentage(equal));
            }
            v
        } else {
            vec![]
        }
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
