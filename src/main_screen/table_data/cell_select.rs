use super::data_table::DataTable;

impl DataTable {
    pub fn select_cell_next(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let row: usize = {
                let new = row + 1;
                if new >= self.width() {
                    0
                } else {
                    new
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    pub fn select_cell_previous(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let row: usize = {
                if row == 0 {
                    self.width()
                } else {
                    row - 1
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }

    pub fn select_cell_down(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let col: usize = {
                let new = col + 1;
                if new >= self.height() {
                    0
                } else {
                    new
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }

    pub fn select_cell_up(&mut self) {
        if let Some((col, row)) = self.table_state.selected_cell() {
            let col: usize = {
                if col == 0 {
                    self.height()
                } else {
                    col - 1
                }
            };
            self.table_state.select_cell(Some((col, row)));
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
}
