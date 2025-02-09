use ratatui::{text::Line, Frame};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use crate::backend::{
    key_bindings::KeyBindings, tasks::events::IoCommand, CsvDescription, IoCommandError,
    IoCommandResult,
};

use super::{layout::header_body_footer_areas, table_data::data_table::DataTable};

/// Application.
#[derive(Debug)]
pub struct App {
    pub key_bindings: KeyBindings,
    pub io_command_sender: UnboundedSender<IoCommand>,
    pub running: bool,
    pub data: DataTable,
    pub io_error: Option<IoCommandError>,
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
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(io_command_sender: UnboundedSender<IoCommand>) -> Self {
        Self {
            key_bindings: KeyBindings::default(),
            io_command_sender,
            running: true,
            // main_screen: MainScreen::default(),
            data: DataTable::default(),
            io_error: None,
        }
    }

    pub fn from_parsed_csv(&mut self, data: IoCommandResult<CsvDescription>) {
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

    pub fn save(&mut self) {
        self.io_command_sender
            .send(self.data.save_command())
            .expect("IoCommand Receiver Closed. Quitting");
    }

    pub fn save_key_bindings(&self) {
        self.io_command_sender
            .send(IoCommand::SaveKeyBindings(self.key_bindings.clone()))
            .expect("IoCommand Receiver Closed. Quitting");
    }

    pub fn reload_key_bindings(&self) {
        self.io_command_sender
            .send(IoCommand::LoadKeyBindings)
            .expect("IoCommand Receiver Closed. Quitting");
    }
    pub fn set_key_bindings(&mut self, key_bindings: IoCommandResult<KeyBindings>) {
        match key_bindings {
            Ok(key_bindings) => self.key_bindings = key_bindings,
            Err(e) => self.io_error = Some(e),
        }
        info!("\n{:#?}", "SET KEYBINDINGS");
    }
}
