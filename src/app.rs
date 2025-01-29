use std::error;

use derive_more::derive::From;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::{main_screen::MainScreen, utils::layout_helpers::header_body_footer_areas};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, From)]
pub enum AppError {
    EditingError,
}
impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum AppMode {
    Main,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub app_mode: AppMode,
    pub main_screen: MainScreen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            app_mode: AppMode::Main,
            main_screen: MainScreen::default(),
        }
    }
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let [_header, body, footer] = header_body_footer_areas(1, 6, frame.area());

        self.render_screen(frame, body);
        self.render_header(frame, _header);

        frame.render_widget(
            Paragraph::new(format!(
                "This is a tui template.\n\
                    Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                    Press left and right to increment and decrement the counter respectively.\n\
                    Counter: {}",
                self.counter
            ))
            .block(
                Block::bordered()
                    .title("Template")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
            footer,
        )
    }
    pub fn render_screen(&mut self, frame: &mut Frame, area: Rect) {
        match self.app_mode {
            AppMode::Main => self.main_screen.render_body(frame, area),
            // AppMode::Main(_) => MainScreen::render_body(self, frame, area),
        }
    }
    pub fn render_header(&mut self, frame: &mut Frame, area: Rect) {
        match self.app_mode {
            AppMode::Main => {
                let text = format!("AppMode: {:?}", self.app_mode);
                let line = Line::from(text);
                frame.render_widget(line, area);
            }
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
