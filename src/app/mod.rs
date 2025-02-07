use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

#[allow(unused)]
use tracing::info;

use crate::{
    event::io_task::IoTask, main_screen::MainScreen,
    utils::layout_helpers::header_body_footer_areas,
};

pub mod evt_handlers;

#[derive(Debug)]
pub enum AppMode {
    Main,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub action_sender: UnboundedSender<IoTask>,
    pub running: bool,
    pub app_mode: AppMode,
    pub main_screen: MainScreen,
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let [_header, _body, _footer] = header_body_footer_areas(1, 6, frame.area());
        // info!("{:#?}", "RENDER");
        match self.app_mode {
            AppMode::Main => self.main_screen.render_body(frame, frame.area()),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(action_sender: UnboundedSender<IoTask>) -> Self {
        Self {
            action_sender,
            running: true,
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
}
