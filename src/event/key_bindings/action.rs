use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    ToggleEdit,
    Save,
}

#[allow(unused)]
impl Action {
    pub fn act(&self, app: &mut App) {
        match self {
            Self::ToggleEdit => {
                app.data.toggle_edit();
            }
            Self::Save => {
                app.data.action_save();
            }
        }
    }
}
