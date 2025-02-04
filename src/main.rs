#![feature(arbitrary_self_types_pointers)]
#![feature(arbitrary_self_types)]

use std::io;

use clap::Parser;
use event::Action;
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing::info;

use crate::{
    app::App,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};
pub use error::{AppError, AppResult};

pub mod app;
pub mod cli;
mod error;
pub mod event;
pub mod handler;
pub mod main_screen;
pub mod tui;
pub mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = cli::Cli::parse();
    utils::logging::EzLog::init()?;
    // info!("CLI: \n{:#?}", cli.path.unwrap().path());
    // info!("CLI: \n{:#?}", cli.delim);

    let events = EventHandler::new(250);

    let mut app = App::new(events.action_sender());

    if let Some(path) = cli.path {
        events
            .action_sender()
            .send(Action::LoadCsv {
                path: path.path().to_owned(),
                delim: cli.delim,
            })
            .unwrap();
    }

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    info!("{:#?}", "Starting main loop");
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::TableData(data) => app.main_screen.data_table.set_data(data),
            Event::ReadCsvString { data, path, delim } => app
                .main_screen
                .data_table
                .from_csv_string(data, path, delim),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
