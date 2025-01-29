use ratatui::{layout::Rect, text::Line, Frame};

use crate::{main_screen::MainScreen, utils::layout_helpers::header_body_footer_areas};

mod tmp;

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
        let [header, body, footer] = header_body_footer_areas(1, 6, frame.area());

        self.render_header(frame, header);
        self.render_body(frame, body);
        frame.render_widget(tmp::original(self), footer)
    }

    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        match self.app_mode {
            AppMode::Main => self.main_screen.render_body(frame, area),
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
