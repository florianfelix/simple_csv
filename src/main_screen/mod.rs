use crossterm::event::{KeyCode, KeyEvent};
use itertools::Itertools;
use object::Fields;
use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tracing::info;
use transaction::Transaction;

use crate::{
    app::App, handler::base_key_events, utils::layout_helpers::triple_pane_percantages, AppResult,
};

pub mod object;
pub mod transaction;

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
    pub fields: Fields,
    pub transactions: Vec<Transaction>,
    pub current_ta: Option<Transaction>,
    pub editing: Option<TaField>,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            buffer: String::from("Buffer"),
            name: String::from("Main Screen"),
            fields: Fields::default(),
            transactions: vec![],
            current_ta: Some(Transaction::default()),
            editing: Some(TaField::Name),
        }
    }
}

impl MainScreen {
    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        let [left, center, right] = triple_pane_percantages(20, 40, 40, area);

        self.render_fields(frame, left);
        self.render_buffer(frame, center);

        if let Some(ta) = self.current_ta.clone() {
            ta.render_as_table(frame, right);
        }
    }
    fn render_fields(&mut self, frame: &mut Frame, area: Rect) {
        let lines = self
            .fields
            .0
            .iter()
            .map(|f| Line::from(f.name()))
            .collect_vec();

        let w = Paragraph::new(lines);
        frame.render_widget(w, area);
    }
    fn render_buffer(&mut self, frame: &mut Frame, area: Rect) {
        let title = format!("{:} - {:?}", self.buffer, self.editing);
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(title);

        let text = Paragraph::new(self.buffer.as_str()).block(block);
        frame.render_widget(text, area);
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
                KeyCode::Char('j') => info!("\n{:#?}", app.main_screen.fields.to_json()),
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }
}
