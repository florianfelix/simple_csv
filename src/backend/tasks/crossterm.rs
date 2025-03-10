use crossterm::event::Event as CrosstermEvent;
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

#[allow(unused)]
use tracing::info;

use super::events::BackendEvent;

pub async fn crossterm_task(
    tick_rate: Duration,
    event_sender: mpsc::UnboundedSender<BackendEvent>,
) {
    let mut reader = crossterm::event::EventStream::new();
    let mut tick = tokio::time::interval(tick_rate);

    loop {
        let _tick_delay = tick.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = event_sender.closed() => {
            break;
          }
          // _ = _tick_delay => {
          //   event_sender.send(BackendEvent::Tick).unwrap();
          // }

          Some(Ok(evt)) = crossterm_event => {
            match evt {
              CrosstermEvent::Key(key) => {
                if key.kind == crossterm::event::KeyEventKind::Press {
                  event_sender.send(BackendEvent::Key(key)).unwrap();
                }
              },
              CrosstermEvent::Mouse(mouse) => {
                event_sender.send(BackendEvent::Mouse(mouse)).unwrap();
              },
              CrosstermEvent::Resize(x, y) => {
                event_sender.send(BackendEvent::Resize(x, y)).unwrap();
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
