#[allow(unused)]
use tracing::info;

use super::{extensions::TableExt, DataTable};

impl DataTable {
    pub fn select_cell_right(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            let col: usize = {
                let new = col + 1;
                if new >= self.df.width() {
                    0
                } else {
                    new
                }
            };
            if self.df.is_valid(row, col) {
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
                    self.df.width() - 1
                } else {
                    col - 1
                }
            };
            if self.df.is_valid(row, col) {
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
                if new >= self.df.height() {
                    0
                } else {
                    new
                }
            };
            if self.df.is_valid(row, col) {
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
                    self.df.height() - 1
                } else {
                    row - 1
                }
            };
            if self.df.is_valid(row, col) {
                self.table_state.select_cell(Some((row, col)));
            }
        } else {
            self.table_state.select_cell(Some((0, 0)));
        }
    }
}
