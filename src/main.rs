use backend::{event_handler::EventHandler, tasks::events::IoCommand};
use clap::Parser;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::info;

use crate::{app::App, tui::Tui};
pub use error::{AppError, AppResult};

pub mod app;
pub mod backend;
mod error;
pub mod tui;
pub mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    #[cfg(debug_assertions)]
    utils::logging::EzLog::init()?;

    let cli = utils::cli::Cli::parse();

    let events = EventHandler::new(250);

    if let Some(ref path) = cli.path {
        events
            .io_command_sender()
            .send(IoCommand::LoadCsv {
                path: path.path().to_owned(),
                delim: cli.delim,
            })
            .unwrap();
    }

    let mut app = App::new(events.io_command_sender(), cli);

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
        app.handle_backend_events(tui.events.next().await?);
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
