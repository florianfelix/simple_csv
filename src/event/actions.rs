use std::path::PathBuf;

use tokio::{io::AsyncReadExt, sync::mpsc};
use tracing::info;

use crate::{event::csv::CsvFileDescription, AppResult};

use super::crossterm::Event;

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
