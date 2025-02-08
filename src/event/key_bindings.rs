#[allow(unused)]
use tracing::info;

use super::IoTaskResult;

mod utils;

pub struct KeyBindings;

impl KeyBindings {
    pub fn load() -> IoTaskResult<()> {
        info!("{:#?}", "LOADING KEYBINDINGS");

        Ok(())
    }
}
