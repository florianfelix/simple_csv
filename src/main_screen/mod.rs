use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{App, AppMode, AppResult},
    handler::base_key_events,
};

#[derive(Debug)]
pub enum TaField {
    Name,
}

#[derive(Debug)]
pub struct Transaction {
    pub name: String,
}
impl Default for Transaction {
    fn default() -> Self {
        Self {
            name: String::from("Transaction Name"),
        }
    }
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
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Editing");

        let t = app.main_screen.name.clone();
        let text = Paragraph::new(t).block(block);
        frame.render_widget(text, area);
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
