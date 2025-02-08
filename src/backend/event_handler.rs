use std::{path::PathBuf, time::Duration};

use notify_debouncer_full::{notify::INotifyWatcher, Debouncer, NoCache};
use tokio::sync::mpsc;
#[allow(unused)]
use tracing::info;

use crate::AppResult;

use super::tasks::{
    crossterm::crossterm_task,
    events::{BackendEvent, IoCommand},
    io_task::io_task,
};

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    event_sender: mpsc::UnboundedSender<BackendEvent>,
    /// Event receiver channel.
    event_receiver: mpsc::UnboundedReceiver<BackendEvent>,
    /// Event handler thread.
    event_handler: tokio::task::JoinHandle<()>,
    /// IoCommand sender channel.
    io_command_sender: mpsc::UnboundedSender<IoCommand>,
    /// IoCommand handler thread.
    io_command_handler: tokio::task::JoinHandle<()>,
    watcher: Debouncer<INotifyWatcher, NoCache>,
}

impl EventHandler {
    pub fn io_command_sender(&self) -> mpsc::UnboundedSender<IoCommand> {
        self.io_command_sender.clone()
    }
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64, path: PathBuf) -> Self {
        // Events
        let tick_rate = Duration::from_millis(tick_rate);
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let _event_sender = event_sender.clone();
        let event_handler = tokio::spawn(crossterm_task(tick_rate, _event_sender));

        // Io Task
        let (io_command_sender, io_command_receiver) = mpsc::unbounded_channel::<IoCommand>();
        let _event_sender = event_sender.clone();
        let io_command_handler = tokio::spawn(io_task(_event_sender, io_command_receiver));

        // Watcher
        let watcher = Self::watch(io_command_sender.clone(), path).unwrap();

        Self {
            event_sender,
            event_receiver,
            event_handler,
            io_command_sender,
            io_command_handler,
            watcher,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub async fn next(&mut self) -> AppResult<BackendEvent> {
        self.event_receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "This is an IO error",
            )))
    }
}

impl EventHandler {
    pub fn watch(
        sender: mpsc::UnboundedSender<IoCommand>,
        path: PathBuf,
    ) -> AppResult<
        notify_debouncer_full::Debouncer<
            notify_debouncer_full::notify::INotifyWatcher,
            notify_debouncer_full::NoCache,
        >,
    > {
        let mut debouncer = notify_debouncer_full::new_debouncer(
            std::time::Duration::from_secs_f32(0.5),
            None,
            move |event: notify_debouncer_full::DebounceEventResult| match event {
                Ok(events) => events.iter().for_each(|event| {
                    if let notify_debouncer_full::DebouncedEvent {
                        event:
                            notify_debouncer_full::notify::Event {
                                kind: notify_debouncer_full::notify::EventKind::Modify(_),
                                ..
                            },
                        ..
                    } = event
                    {
                        sender.send(IoCommand::LoadKeyBindings).unwrap()
                    }
                }),
                Err(_errors) => {}
            },
        )?;
        debouncer.watch(
            path,
            notify_debouncer_full::notify::RecursiveMode::Recursive,
        )?;
        Ok(debouncer)
    }
}
