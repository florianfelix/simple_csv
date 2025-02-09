use tokio::sync::mpsc;

#[allow(unused)]
use tracing::info;

use crate::backend::{csv::load_csv, key_bindings::KeyBindings, utils::save_file};

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
                        event_sender.send(BackendEvent::ParsedCsv(parsed)).unwrap();
                    },
                    IoCommand::SaveCsv(data) => {
                        let content = data.data_to_string().unwrap();
                        save_file(&data.path.unwrap(), &content).await.unwrap();
                    }
                    IoCommand::LoadKeyBindings => {
                        let key_bindings = KeyBindings::load().await;
                        event_sender.send(BackendEvent::LoadedKeybindings(key_bindings)).unwrap()
                    }
                    IoCommand::SaveKeyBindings(key_bindings) => {
                        key_bindings.save().await.unwrap();
                    }
                }
            }
        }
    }
}
