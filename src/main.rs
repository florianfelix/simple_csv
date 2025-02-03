#![feature(arbitrary_self_types_pointers)]
#![feature(arbitrary_self_types)]

use std::io;

use clap::Parser;
use event::Action;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;
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
    // Create an application.
    let (action_sender, action_receiver) = mpsc::unbounded_channel::<Action>();
    let mut app = App::new(action_sender.clone());

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250, action_receiver);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    if let Some(path) = cli.path {
        action_sender
            .send(Action::LoadCsv {
                path: path.path().to_owned(),
                delim: cli.delim,
            })
            .unwrap();
    }
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
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
