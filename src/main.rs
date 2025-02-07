use clap::Parser;
use event::{crossterm::Event, event_handler::EventHandler, io_task::IoTask};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::info;

use crate::{app::App, tui::Tui};
pub use error::{AppError, AppResult};

pub mod app;
pub mod cli;
mod error;
pub mod event;
// pub mod main_screen;
pub mod tui;
pub mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = cli::Cli::parse();
    utils::logging::EzLog::init()?;

    let events = EventHandler::new(250);

    let mut app = App::new(events.io_task_sender());

    if let Some(path) = cli.path {
        events
            .io_task_sender()
            .send(IoTask::LoadCsv {
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
            Event::Key(key_event) => app.handle_key_events(key_event)?,
            // Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::ParsedCsv(parsed) => app.from_parsed_csv(parsed),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
