use std::{io::Read, path::PathBuf, time::Duration};

use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tracing::info;

use crate::{main_screen::table_data::io::headers_rows_from_csv_string, AppResult};

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
}

#[derive(Clone, Debug)]
pub enum Action {
    SaveToFile(String),
    LoadCsv { path: PathBuf, delim: char },
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
    handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64, mut action_receiver: mpsc::UnboundedReceiver<Action>) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let _event_sender = event_sender.clone();
        let handler = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);
            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  _ = _event_sender.closed() => {
                    break;
                  }
                  _ = tick_delay => {
                    _event_sender.send(Event::Tick).unwrap();
                  }
                  action = action_receiver.recv() => {
                      info!("{:#?}", action);
                      if let Some(action) = action {
                          match action {
                              Action::LoadCsv{path, delim} => {
                                  let mut file = std::fs::File::open(path).unwrap();
                                  let mut buffer = String::new();
                                  file.read_to_string(&mut buffer).unwrap();
                                  let data = headers_rows_from_csv_string(&buffer, delim);
                                  _event_sender.send(Event::TableData(data)).unwrap();
                              },
                              _ => {}
                          }
                      }
                      }
                  Some(Ok(evt)) = crossterm_event => {
                    match evt {
                      CrosstermEvent::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                          _event_sender.send(Event::Key(key)).unwrap();
                        }
                      },
                      CrosstermEvent::Mouse(mouse) => {
                        _event_sender.send(Event::Mouse(mouse)).unwrap();
                      },
                      CrosstermEvent::Resize(x, y) => {
                        _event_sender.send(Event::Resize(x, y)).unwrap();
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
        });
        Self {
            event_sender,
            event_receiver,
            handler,
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
