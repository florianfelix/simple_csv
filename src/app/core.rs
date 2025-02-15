use crokey::Combiner;
use ratatui::{text::Line, Frame};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;

use crate::{
    backend::{
        file_formats::file_csv::CsvDescription, key_bindings::KeyBindings,
        tasks::events::IoCommand, IoCommandError, IoCommandResult,
    },
    utils::cli::Cli,
};

use super::{
    component_keybindings::KeyBindingsDisplay, component_table::DataTable,
    layout::header_body_footer_areas,
};

/// Application.
#[derive(Debug)]
pub struct App {
    pub cli: Cli,
    pub key_bindings: KeyBindings,
    pub combiner: Combiner,
    pub io_command_sender: UnboundedSender<IoCommand>,
    pub running: bool,
    pub data: DataTable,
    pub io_error: Option<IoCommandError>,
    pub show_key_bindings: bool,
    pub key_bindings_display: KeyBindingsDisplay,
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [_header, _body, _footer] = header_body_footer_areas(1, 6, frame.area());
        // info!("{:#?}", "RENDER");

        if self.show_key_bindings {
            self.key_bindings_display
                .render(frame, area, &self.key_bindings);
            return;
        }

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
    pub fn new(io_command_sender: UnboundedSender<IoCommand>, cli: Cli) -> Self {
        Self {
            cli,
            key_bindings: KeyBindings::default(),
            combiner: Combiner::default(),
            io_command_sender,
            running: true,
            data: DataTable::default(),
            io_error: None,
            show_key_bindings: false,
            key_bindings_display: KeyBindingsDisplay::default(),
        }
    }

    pub fn from_parsed_csv(&mut self, data: IoCommandResult<CsvDescription>) {
        match data {
            Ok(csv_description) => {
                self.io_error = None;
                self.data.from_csv_description(csv_description);
            }
            Err(e) => {
                self.io_error = Some(e);
                self.data = DataTable::default();
                match self.cli.path {
                    Some(ref cliopath) => self.data.path = Some(cliopath.path().to_owned()),
                    None => self.data.path = None,
                }
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
            .send(self.data.save_csv_command())
            .expect("IoCommand Receiver Closed. Quitting");
    }

    pub fn save_as_toml(&mut self) {
        self.io_command_sender
            .send(self.data.save_toml_command())
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
        info!("{:#?}", "SET KEYBINDINGS");
    }
    pub fn toggle_keybindings(&mut self) {
        self.show_key_bindings = !self.show_key_bindings;
    }
}
