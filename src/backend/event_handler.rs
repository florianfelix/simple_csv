use std::time::Duration;

use tokio::sync::mpsc;
#[allow(unused)]
use tracing::info;

use crate::AppResult;

use super::tasks::{
    crossterm::{crossterm_task, BackendEvent},
    io_task::{io_task, IoTask},
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
    /// IoTask sender channel.
    io_task_sender: mpsc::UnboundedSender<IoTask>,
    /// IoTask handler thread.
    action_handler: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    pub fn io_task_sender(&self) -> mpsc::UnboundedSender<IoTask> {
        self.io_task_sender.clone()
    }
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        // Events
        let tick_rate = Duration::from_millis(tick_rate);
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let _event_sender = event_sender.clone();
        let event_handler = tokio::spawn(crossterm_task(tick_rate, _event_sender));

        // Io Task
        let (io_task_sender, io_task_receiver) = mpsc::unbounded_channel::<IoTask>();
        let _event_sender = event_sender.clone();
        let action_handler = tokio::spawn(io_task(_event_sender, io_task_receiver));

        Self {
            event_sender,
            event_receiver,
            event_handler,
            io_task_sender,
            action_handler,
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
