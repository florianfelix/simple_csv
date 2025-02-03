use ratatui::{layout::Rect, text::Line, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    event::Action, main_screen::MainScreen, utils::layout_helpers::header_body_footer_areas,
};

mod tmp;

#[derive(Debug)]
pub enum AppMode {
    Main,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub action_sender: UnboundedSender<Action>,
    pub running: bool,
    pub counter: u8,
    pub app_mode: AppMode,
    pub main_screen: MainScreen,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             running: true,
//             counter: 0,
//             app_mode: AppMode::Main,
//             main_screen: MainScreen::default(),
//         }
//     }
// }

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let [_header, _body, _footer] = header_body_footer_areas(1, 6, frame.area());
        // TODO: move into main_screen
        // self.render_header(frame, header);
        self.render_body(frame, frame.area());
        // self.render_footer(frame, footer);
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

    pub fn render_body(&mut self, frame: &mut Frame, area: Rect) {
        match self.app_mode {
            AppMode::Main => self.main_screen.render_body(frame, area),
        }
    }

    pub fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        match self.app_mode {
            AppMode::Main => frame.render_widget(tmp::original(self), area),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(action_sender: UnboundedSender<Action>) -> Self {
        Self {
            action_sender,
            running: true,
            counter: 0,
            app_mode: AppMode::Main,
            main_screen: MainScreen::default(),
        }
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
