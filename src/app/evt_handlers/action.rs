use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Action {
    Quit,
    EditCell,
    EditHeader,
    EditFileName,
    ApplyEdit,
    CancelEdit,
    NextSuggestion,
    CursorRight,
    CursorLeft,
    Save,
    SaveKeyBindings,
    SelectCellRight,
    SelectCellLeft,
    SelectCellUp,
    SelectCellDown,
    SelectFirstRow,
    SelectLastRow,
    ConfirmSelectCellRight,
    ConfirmSelectCellLeft,
    ConfirmSelectCellUp,
    ConfirmSelectCellDown,
    AppendRow,
    AppendColumn,
    ToggleKeyBindingsDisplay,
    SaveToml,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
            CancelEdit => self.data.edit_cancel(),
            NextSuggestion => self.data.skim_select_next(),
            CursorRight => self.data.move_cursor_right(),
            CursorLeft => self.data.move_cursor_left(),
            Save => self.save(),
            SaveKeyBindings => self.save_key_bindings(),
            SelectCellRight => self.data.select_cell_right(),
            SelectCellLeft => self.data.select_cell_left(),
            SelectCellUp => self.data.select_cell_up(),
            SelectCellDown => self.data.select_cell_down(),
            SelectFirstRow => self.data.table_state.select_first(),
            SelectLastRow => self.data.table_state.select_last(),
            ConfirmSelectCellRight => {
                self.data.apply_edit();
                self.data.select_cell_right();
            }
            ConfirmSelectCellLeft => {
                self.data.apply_edit();
                self.data.select_cell_left();
            }
            ConfirmSelectCellUp => {
                self.data.apply_edit();
                self.data.select_cell_up();
            }
            ConfirmSelectCellDown => {
                self.data.apply_edit();
                self.data.select_cell_down();
            }
            AppendRow => self.data.append_row(),
            AppendColumn => self.data.append_column(),
            ToggleKeyBindingsDisplay => self.toggle_keybindings(),
            SaveToml => self.save_as_toml(),
        }
    }
}
