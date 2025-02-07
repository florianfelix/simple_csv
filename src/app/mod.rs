use ratatui::{text::Line, Frame};
use table_data::data_table::DataTable;
use tokio::sync::mpsc::UnboundedSender;

#[allow(unused)]
use tracing::info;

use crate::{
    event::{csv::CsvDescription, io_task::IoTask, IoTaskError, IoTaskResult},
    utils::layout_helpers::header_body_footer_areas,
};

pub mod evt_handlers;
pub mod table_data;

/// Application.
#[derive(Debug)]
pub struct App {
    pub action_sender: UnboundedSender<IoTask>,
    pub running: bool,
    // pub main_screen: MainScreen,
    pub data: DataTable,
    pub io_error: Option<IoTaskError>,
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [_header, _body, _footer] = header_body_footer_areas(1, 6, frame.area());
        // info!("{:#?}", "RENDER");
        if self.data.width() > 0 {
            self.data.render(frame, area);
        } else if let Some(e) = &self.io_error {
            let txt = Line::from(e.to_string());
            frame.render_widget(txt, area);
        } else {
            frame.render_widget(Line::from("No data"), area);
        }

        // self.main_screen.render_body(frame, frame.area());
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(action_sender: UnboundedSender<IoTask>) -> Self {
        Self {
            action_sender,
            running: true,
            // main_screen: MainScreen::default(),
            data: DataTable::default(),
            io_error: None,
        }
    }

    pub fn from_parsed_csv(&mut self, data: IoTaskResult<CsvDescription>) {
        match data {
            Ok(csv) => {
                self.io_error = None;
                self.data = DataTable::default()
                    .set_headers(csv.data.headers)
                    .set_rows(csv.data.rows)
                    .set_parse_errors(csv.errors)
                    .set_path(csv.path)
                    .set_delim(csv.delim);
            }
            Err(e) => {
                // panic!("{}", e);
                self.io_error = Some(e);
                self.data = DataTable::default();
            }
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
