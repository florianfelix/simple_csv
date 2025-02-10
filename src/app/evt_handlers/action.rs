use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Quit,
    EditCell,
    EditHeader,
    EditFileName,
    ApplyEdit,
    CursorRight,
    CursorLeft,
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
            EditCell => self.data.edit_cell(),
            EditHeader => self.data.edit_column_name(),
            EditFileName => self.data.edit_file_name(),
            ApplyEdit => self.data.apply_edit(),
            CursorRight => self.data.move_cursor_right(),
            CursorLeft => self.data.move_cursor_left(),
            Save => self.save(),
            SaveKeyBindings => self.save_key_bindings(),
            SelectRight => self.data.select_cell_right(),
            SelectLeft => self.data.select_cell_left(),
            SelectUp => self.data.select_cell_up(),
            SelectDown => self.data.select_cell_down(),
            SelectFirstRow => self.data.table_state.select_first(),
            SelectLastRow => self.data.table_state.select_last(),
            ConfirmSelectRight => {
                self.data.apply_edit();
                self.data.select_cell_right();
            }
            ConfirmSelectLeft => {
                self.data.apply_edit();
                self.data.select_cell_left();
            }
            ConfirmSelectUp => {
                self.data.apply_edit();
                self.data.select_cell_up();
            }
            ConfirmSelectDown => {
                self.data.apply_edit();
                self.data.select_cell_down();
            }
            AppendRow => self.data.append_row(),
            AppendColumn => self.data.append_column(),
        }
    }
}
