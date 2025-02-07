use crossterm::event::{Event as CrosstermEvent, KeyEvent, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

use super::{csv::CsvDescription, ActionResult};

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    ParsedCsv(ActionResult<CsvDescription>),
    // TableData((Vec<String>, Vec<Vec<String>>)),
    // LoadedCsv(CsvFileDescription),
}

pub async fn crossterm_task(tick_rate: Duration, event_sender: mpsc::UnboundedSender<Event>) {
    let mut reader = crossterm::event::EventStream::new();
    let mut tick = tokio::time::interval(tick_rate);

    loop {
        let _tick_delay = tick.tick();
        let crossterm_event = reader.next().fuse();
        tokio::select! {
          _ = event_sender.closed() => {
            break;
          }
          // _ = tick_delay => {
          //   event_sender.send(Event::Tick).unwrap();
          // }

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
