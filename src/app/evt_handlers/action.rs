use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Quit,
    ToggleEdit,
    ModeEdit,
    ModeNormal,
    Save,
    SaveKeyBindings,
    SelectRight,
    SelectLeft,
    SelectUp,
    SelectDown,
    SelectFirstRow,
    SelectLastRow,
    ConfirmSelectRight,
    ConfirmSelectLeft,
    ConfirmSelectUp,
    ConfirmSelectDown,
    AppendRow,
    AppendColumn,
}

impl App {
    pub fn perform_action(&mut self, action: Action) {
        use Action::*;
        match action {
            Quit => self.quit(),
            ToggleEdit => self.data.toggle_edit(),
            ModeEdit => self.data.mode_edit(),
            ModeNormal => self.data.mode_normal(),
            Save => self.save(),
            SaveKeyBindings => self.save_key_bindings(),
            SelectRight => self.data.select_cell_right(),
            SelectLeft => self.data.select_cell_left(),
            SelectUp => self.data.select_cell_up(),
            SelectDown => self.data.select_cell_down(),
            SelectFirstRow => self.data.table_state.select_first(),
            SelectLastRow => self.data.table_state.select_last(),
            ConfirmSelectRight => {
                self.data.mode_normal();
                self.data.select_cell_right();
            }
            ConfirmSelectLeft => {
                self.data.mode_normal();
                self.data.select_cell_left();
            }
            ConfirmSelectUp => {
                self.data.mode_normal();
                self.data.select_cell_up();
            }
            ConfirmSelectDown => {
                self.data.mode_normal();
                self.data.select_cell_down();
            }
            AppendRow => self.data.append_row(),
            AppendColumn => self.data.append_column(),
        }
    }
}
