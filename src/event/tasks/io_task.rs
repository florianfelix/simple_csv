use std::path::PathBuf;
use tokio::sync::mpsc;

#[allow(unused)]
use tracing::info;

use super::crossterm::Event;
use crate::event::{
    csv::{load_csv, CsvDescription},
    key_bindings::KeyBindingsIo,
    utils::save_file,
};

#[derive(Clone, Debug)]
pub enum IoTask {
    SaveCsv(CsvDescription),
    LoadCsv { path: PathBuf, delim: char },
    LoadKeyBindings,
}

pub async fn io_task(
    event_sender: mpsc::UnboundedSender<Event>,
    mut io_task_receiver: mpsc::UnboundedReceiver<IoTask>,
) {
    loop {
        tokio::select! {
            _ = event_sender.closed() => {
              break;
            }
            Some(io_task) = io_task_receiver.recv() => {
                // info!("{:#?}", io_task);
                match io_task {
                    IoTask::LoadCsv{path, delim} => {
                        let parsed = load_csv(path, delim).await;
                        event_sender.send(Event::ParsedCsv(parsed)).unwrap();
                    },
                    IoTask::SaveCsv(data) => {
                        let content = data.data_to_string().unwrap();
                        save_file(&data.path.unwrap(), &content).await.unwrap();
                    }
                    IoTask::LoadKeyBindings => {
                        let _key_bindings = KeyBindingsIo::load().await;
                        // info!("{:#?}", _key_bindings);
                    }
                }
            }
        }
    }
}
