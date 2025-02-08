// #![feature(inherent_associated_types)]
// #![allow(incomplete_features)]

use backend::{
    event_handler::EventHandler,
    key_bindings::KeyBindingsIo,
    tasks::events::{BackendEvent, IoCommand},
};
use clap::Parser;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tracing::info;

use crate::{app::App, tui::Tui};
pub use error::{AppError, AppResult};

pub mod app;
pub mod backend;
mod error;
// pub mod tmp;
pub mod tui;
pub mod utils;

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = utils::cli::Cli::parse();
    utils::logging::EzLog::init()?;

    // ensure keybindings file exists
    let _ = KeyBindingsIo::load().await;
    let keymap = KeyBindingsIo::get_conf_path().await.unwrap();

    let events = EventHandler::new(250, keymap);

    let mut app = App::new(events.io_command_sender());

    if let Some(path) = cli.path {
        events
            .io_command_sender()
            .send(IoCommand::LoadCsv {
                path: path.path().to_owned(),
                delim: cli.delim,
            })
            .unwrap();
    }
    events
        .io_command_sender()
        .send(IoCommand::LoadKeyBindings)
        .expect("IoCommand Receiver Closed. Quitting");

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    info!("{:#?}", "Starting main loop");
    // tmp::serialise_test().await;
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            BackendEvent::Tick => app.tick(),
            BackendEvent::Key(key_event) => app.handle_key_events(key_event)?,
            BackendEvent::Mouse(_) => {}
            BackendEvent::Resize(_, _) => {}
            BackendEvent::ParsedCsv(parsed) => app.from_parsed_csv(parsed),
            BackendEvent::LoadedKeybindings(key_bindings) => app.set_key_bindings(key_bindings),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
