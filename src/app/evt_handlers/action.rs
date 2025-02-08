use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    ToggleEdit,
    Save,
    SelectRight,
    SelectLeft,
    SelectUp,
    SelectDown,
    SelectFirstRow,
    SelectLastRow,
    AppendRow,
    ConfirmSelectRight,
    ConfirmSelectLeft,
    ConfirmSelectUp,
    ConfirmSelectDown,
}

impl App {
    pub fn perform_action(&mut self, action: Action) {
        use Action::*;
        match action {
            ToggleEdit => self.data.toggle_edit(),
            Save => self.save(),
            SelectRight => self.data.select_cell_right(),
            SelectLeft => self.data.select_cell_left(),
            SelectUp => self.data.select_cell_up(),
            SelectDown => self.data.select_cell_down(),
            SelectFirstRow => self.data.table_state.select_first(),
            SelectLastRow => self.data.table_state.select_last(),
            AppendRow => self.data.append_row(),
            ConfirmSelectRight => {
                self.data.toggle_edit();
                self.data.select_cell_right();
            }
            ConfirmSelectLeft => {
                self.data.toggle_edit();
                self.data.select_cell_left();
            }
            ConfirmSelectUp => {
                self.data.toggle_edit();
                self.data.select_cell_up();
            }
            ConfirmSelectDown => {
                self.data.toggle_edit();
                self.data.select_cell_down();
            }
        }
    }
}
