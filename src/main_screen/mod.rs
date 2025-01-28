use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use transaction::Transaction;

use crate::{
    app::{App, AppMode, AppResult},
    handler::base_key_events,
};

pub mod transaction;

#[derive(Debug)]
pub enum TaField {
    Name,
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
    pub current_ta: Option<Transaction>,
    pub editing: Option<TaField>,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self {
            buffer: String::from(""),
            name: String::from("Main Screen"),
            current_ta: Some(Transaction::default()),
            editing: Some(TaField::Name),
        }
    }
}

impl MainScreen {
    pub fn render(app: &mut App, frame: &mut Frame, area: Rect) {
        let [left, right] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Editing");

        let t = app.main_screen.name.clone();
        let text = Paragraph::new(t).block(block);
        frame.render_widget(text, left);

        if let Some(ta) = app.main_screen.current_ta.clone() {
            ta.render_self(frame, right);
            // frame.render_widget(ta, right);
        }
    }

    pub fn key_handler_edit(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        if let crossterm::event::KeyEventKind::Press = key_event.kind {
            let edit = &mut app.main_screen.name;

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
                _ => base_key_events(key_event, app)?,
            }
        }
        Ok(())
    }
}
