use std::path::PathBuf;
use tokio::sync::mpsc;
use tracing::info;

use super::{crossterm::Event, csv::CsvDescription};
use crate::event::csv::{load_csv, save_file};

#[derive(Clone, Debug)]
pub enum Action {
    SaveCsv(CsvDescription),
    LoadCsv { path: PathBuf, delim: char },
}

pub async fn action_task(
    event_sender: mpsc::UnboundedSender<Event>,
    mut action_receiver: mpsc::UnboundedReceiver<Action>,
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
                        let parsed = load_csv(path, delim).await;
                        event_sender.send(Event::ParsedCsv(parsed)).unwrap();
                    },
                    Action::SaveCsv(data) => {
                        let content = data.data_to_string().unwrap();
                        save_file(&data.path.unwrap(), &content).await.unwrap();
                    }
                }
            }
        }
    }
}
