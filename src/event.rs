use std::{path::PathBuf, time::Duration};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::{
    io::AsyncReadExt,
    sync::mpsc::{self, UnboundedReceiver},
};
use tracing::info;

use crate::{main_screen::table_data::io::CsvFileDescription, AppResult};

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
    TableData((Vec<String>, Vec<Vec<String>>)),
    LoadedCsv(CsvFileDescription),
    // ReadCsvString {
    //     data: String,
    //     path: PathBuf,
    //     delim: char,
    // },
}

#[derive(Clone, Debug)]
pub enum Action {
    SaveCsv {
        path: PathBuf,
        data: String,
        delim: char,
    },
    LoadCsv {
        path: PathBuf,
        delim: char,
    },
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    event_sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    event_receiver: mpsc::UnboundedReceiver<Event>,
    /// Event handler thread.
    event_handler: tokio::task::JoinHandle<()>,
    /// Action sender channel.
    action_sender: mpsc::UnboundedSender<Action>,
    /// Action handler thread.
    action_handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn action_sender(&self) -> mpsc::UnboundedSender<Action> {
        self.action_sender.clone()
    }
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        // Events
        let tick_rate = Duration::from_millis(tick_rate);
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let _event_sender = event_sender.clone();
        let event_handler = tokio::spawn(crossterm_task(tick_rate, _event_sender));

        // Actions
        let (action_sender, action_receiver) = mpsc::unbounded_channel::<Action>();
        let _event_sender = event_sender.clone();
        let action_handler = tokio::spawn(action_task(_event_sender, action_receiver));

        Self {
            event_sender,
            event_receiver,
            event_handler,
            action_sender,
            action_handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<Event> {
        self.event_receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            )))
    }
}

async fn action_task(
    event_sender: mpsc::UnboundedSender<Event>,
    mut action_receiver: UnboundedReceiver<Action>,
) {
    loop {
        tokio::select! {
            _ = event_sender.closed() => {
              break;
            }
            Some(action) = action_receiver.recv() => {
                info!("{:#?}", action);
                match action {
                    Action::LoadCsv{path, delim} => {
                        let csv_str = read_to_string(&path).await.unwrap();
                        event_sender.send(
                            Event::LoadedCsv(CsvFileDescription { path: path, data: csv_str, delim })
                        ).unwrap();
                        // event_sender.send(
                        //     Event::ReadCsvString {
                        //         data: csv_str.clone(),
                        //         path: path.clone(),
                        //         delim,
                        //     }).unwrap();
                    },
                    Action::SaveCsv{path: _, data: _, delim: _} => {}
                }
            }
        }
    }
}

async fn read_to_string(path: &PathBuf) -> AppResult<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}

async fn crossterm_task(tick_rate: Duration, event_sender: mpsc::UnboundedSender<Event>) {
    let mut reader = crossterm::event::EventStream::new();
    let mut tick = tokio::time::interval(tick_rate);
    loop {
        let tick_delay = tick.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = event_sender.closed() => {
            break;
          }
          _ = tick_delay => {
            event_sender.send(Event::Tick).unwrap();
          }
          Some(Ok(evt)) = crossterm_event => {
            match evt {
              CrosstermEvent::Key(key) => {
                if key.kind == crossterm::event::KeyEventKind::Press {
                  event_sender.send(Event::Key(key)).unwrap();
                }
              },
              CrosstermEvent::Mouse(mouse) => {
                event_sender.send(Event::Mouse(mouse)).unwrap();
              },
              CrosstermEvent::Resize(x, y) => {
                event_sender.send(Event::Resize(x, y)).unwrap();
              },
              CrosstermEvent::FocusLost => {
              },
              CrosstermEvent::FocusGained => {
              },
              CrosstermEvent::Paste(_) => {
              },
            }
          }
        };
    }
}
