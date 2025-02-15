#[allow(unused)]
use tracing::info;

use crate::app::table_data::extensions::RowsExt;

use super::DataTable;

impl DataTable {
    pub fn select_cell_right(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            let col: usize = {
                let new = col + 1;
                if new >= self.width() {
                    0
                } else {
                    new
                }
            };
            if self.rows.is_valid_coords(row, col) {
                self.table_state.select_cell(Some((row, col)));
            }
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
    pub fn select_cell_left(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            let col: usize = {
                if col == 0 {
                    self.width()
                } else {
                    col - 1
                }
            };
            if self.rows.is_valid_coords(row, col) {
                self.table_state.select_cell(Some((row, col)));
            }
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }

    pub fn select_cell_down(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            let row: usize = {
                let new = row + 1;
                if new >= self.height() {
                    0
                } else {
                    new
                }
            };
            if self.rows.is_valid_coords(row, col) {
                self.table_state.select_cell(Some((row, col)));
            }
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }

    pub fn select_cell_up(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            let row: usize = {
                if row == 0 {
                    self.rows.len() - 1
                } else {
                    row - 1
                }
            };
            if self.rows.is_valid_coords(row, col) {
                self.table_state.select_cell(Some((row, col)));
            }
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
}
