use std::path::PathBuf;

use tokio::sync::mpsc;

#[allow(unused)]
use tracing::info;

use crate::backend::{
    file_formats::file_csv::load_csv, key_bindings::KeyBindings, tasks::events::IoEvent,
    utils::save_file,
};

use super::events::{BackendEvent, IoCommand};

pub async fn io_task(
    event_sender: mpsc::UnboundedSender<BackendEvent>,
    mut io_command_receiver: mpsc::UnboundedReceiver<IoCommand>,
) {
    loop {
        tokio::select! {
            _ = event_sender.closed() => {
              break;
            }
            Some(io_task) = io_command_receiver.recv() => {
                // info!("{:#?}", io_task);
                match io_task {
                    IoCommand::LoadCsv{path, delim} => {
                        let parsed = load_csv(path, delim).await;
                        let evt = IoEvent::LoadedCsv(parsed);
                        event_sender.send(BackendEvent::IoEvent(evt)).unwrap();
                    },
                    IoCommand::SaveCsv(data) => {
                        let content = data.data_to_string().unwrap();
                        save_file(&data.path.unwrap_or(PathBuf::from("export.csv")), &content).await.unwrap();
                        event_sender.send(BackendEvent::IoEvent(IoEvent::SavedCsv)).unwrap();
                    }
                    IoCommand::LoadKeyBindings => {
                        let key_bindings = KeyBindings::load().await;
                        let evt = IoEvent::LoadedKeybindings(key_bindings);
                        event_sender.send(BackendEvent::IoEvent(evt)).unwrap()
                    }
                    IoCommand::SaveKeyBindings(key_bindings) => {
                        key_bindings.save().await.unwrap();
                    }
                    IoCommand::SaveToml(data) => {
                        let data_string = data.to_toml_string().unwrap();
                        save_file(&data.path, &data_string).await.unwrap();
                    }
                    IoCommand::SaveJson(data) => {
                        let data_string = data.to_json_string().unwrap();
                        save_file(&data.path, &data_string).await.unwrap();
                    }
                }
            }
        }
    }
}
