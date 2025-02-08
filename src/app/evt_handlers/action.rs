use serde::{Deserialize, Serialize};
use tracing::info;

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    ToggleEdit,
    Save,
    SelectRight,
    SelectLeft,
    SelectUp,
    SelectDown,
}

impl App {
    pub fn perform_action(&mut self, action: Action) {
        use Action::*;
        match action {
            ToggleEdit => {
                info!("{:#?}", "Toggle Edit");
                self.data.toggle_edit();
            }
            Save => {
                info!("{:#?}", "Save");
                self.data.action_save();
            }
            SelectRight => {
                self.data.select_cell_next();
            }
            SelectLeft => {
                self.data.select_cell_previous();
            }
            SelectUp => {
                self.data.select_cell_up();
            }
            SelectDown => {
                self.data.select_cell_down();
            }
        }
    }
}
