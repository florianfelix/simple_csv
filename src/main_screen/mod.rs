use crossterm::event::{KeyCode, KeyEvent};
use helpers::triple_pane;
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
    app::{App, AppMode, AppResult},
    handler::base_key_events,
};

pub mod helpers;
pub mod object;
pub mod transaction;

#[derive(Debug)]
pub enum TaField {
    Name,
    Amount,
}

#[derive(Debug)]
pub enum MainScreenMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct MainScreen {
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
    pub fn render_body(app: &mut App, frame: &mut Frame, area: Rect) {
        let data = &mut app.main_screen;
        let [left, center, right] = triple_pane(20, 40, 40, area);

        data.render_buffer(frame, center);

        data.render_fields(frame, left);

        if let Some(ta) = data.current_ta.clone() {
            ta.render_as_table(frame, right);
            // frame.render_widget(ta, right);
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
    pub fn render_header(app: &mut App, frame: &mut Frame, area: Rect) {
        let text = format!("AppMode: {:?}", app.app_mode);
        let line = Line::from(text);
        frame.render_widget(line, area);
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

                KeyCode::Esc => app.app_mode = AppMode::Main(MainScreenMode::Normal),
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }

    pub fn key_handler_normal(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            // let edit = &mut app.main_screen.name;

            match key_event.code {
                KeyCode::Char('e') => app.app_mode = AppMode::Main(MainScreenMode::Editing),
                KeyCode::Char('j') => info!("\n{:#?}", app.main_screen.fields.to_json()),
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }
}
