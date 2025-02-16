use itertools::Itertools;
use text_buffer::Buffer;

use super::{
    extensions::{RowExt, TableExt},
    skim::Skim,
    DataTable, EditTarget,
};

impl DataTable {
    pub fn edit_column_name(&mut self) {
        if let Some(col) = self.table_state.selected_column() {
            self.edit_target = EditTarget::Header(col);
            self.textbuffer = Buffer::from(self.cell_get_header(col));
            // self.textbuffer = Buffer::from(self.headers.get(col).unwrap().clone());
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
        }
    }
    pub fn edit_cell(&mut self) {
        if let Some((row, col)) = self.table_state.selected_cell() {
            self.edit_target = EditTarget::Cell((row, col));
            self.textbuffer = Buffer::from(self.cell_get_row_col(row, col));
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
            let mut sk = Skim::new(self.textbuffer.as_str(), self.rows.get_column(col));
            sk.update(self.textbuffer.as_str());
            self.skim = Some(sk);
        }
    }
    pub fn edit_file_name(&mut self) {
        if let Some(path) = self.path.clone() {
            self.edit_target = EditTarget::FileName;
            self.textbuffer = Buffer::from(path.to_string_lossy().into_owned());
            self.textbuffer.set_cursor(self.textbuffer.len_chars());
        } else {
            self.edit_target = EditTarget::FileName;
            self.textbuffer = Buffer::new();
        }
    }
    pub fn apply_edit(&mut self) {
        match self.edit_target {
            EditTarget::Header(col) => self.set_column_name(col, self.textbuffer.to_string()),
            EditTarget::Cell((row, col)) => {
                let content = match self.skim {
                    Some(ref sk) => {
                        if let Some(suggestion) = sk.selected() {
                            suggestion
                        } else {
                            self.textbuffer.to_string()
                        }
                    }
                    None => self.textbuffer.to_string(),
                };
                self.cell_set_row_col(row, col, content);
                // self.cell_set_row_col(row, col, self.textbuffer.to_string())
            }
            EditTarget::FileName => {
                if self.textbuffer.is_empty() {
                    self.path = None;
                } else {
                    self.path = Some(self.textbuffer.to_string().into());
                };
            }
            EditTarget::None => {}
        }
        self.edit_target = EditTarget::None;
        self.textbuffer = Buffer::new();
        self.skim = None;
    }

    pub fn skim_select_next(&mut self) {
        if let Some(sk) = &mut self.skim {
            sk.select_next();
        }
    }
    pub fn skim_select_previous(&mut self) {
        if let Some(sk) = &mut self.skim {
            sk.select_previous();
        }
    }
    pub fn edit_cancel(&mut self) {
        self.edit_target = EditTarget::None;
        self.textbuffer = Buffer::new();
        self.skim = None;
    }
    pub fn move_cursor_right(&mut self) {
        let current = self.textbuffer.cursor().chars();
        self.textbuffer.set_cursor(current + 1);
    }
    pub fn move_cursor_left(&mut self) {
        let current = self.textbuffer.cursor().chars();
        if let Some(new) = current.checked_sub(1) {
            self.textbuffer.set_cursor(new);
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.textbuffer.insert_char(c);
        if let Some(sk) = &mut self.skim {
            sk.update(self.textbuffer.as_str());
        }
    }
    pub fn delete_backwards(&mut self) {
        self.textbuffer.delete_backwards(1);
        if let Some(sk) = &mut self.skim {
            sk.update(self.textbuffer.as_str());
        }
    }
    pub fn delete_forwards(&mut self) {
        self.textbuffer.delete_forwards(1);
        if let Some(sk) = &mut self.skim {
            sk.update(self.textbuffer.as_str());
        }
    }
    pub fn append_row(&mut self) {
        self.rows.push(vec![String::new(); self.width()]);
        self.table_state.select(Some(self.height()));
    }
    pub fn append_column(&mut self) {
        self.headers.push(String::from("NewColumn"));
        self.rows.append_column();
        self.table_state.select_column(Some(self.width()));
    }
    pub fn move_column_right(&mut self) {
        if let Some(col) = self.table_state.selected_column() {
            let col_right = self.rows.move_column_right(col);
            if col_right.is_some() {
                self.headers.move_right(col);
                self.table_state.select_column(col_right);
            }
        }
    }
    pub fn move_column_left(&mut self) {
        if let Some(col) = self.table_state.selected_column() {
            let col_left = self.rows.move_column_left(col);
            if col_left.is_some() {
                self.headers.move_left(col);
                self.table_state.select_column(col_left);
            }
        }
    }
}

impl DataTable {
    fn set_dirty(&mut self) {
        self.is_dirty = true;
        self.parse_errors = vec![];
    }
    fn cell_set_row_col(&mut self, row: usize, col: usize, content: String) {
        if self.rows.is_valid_coords(row, col) {
            self.rows.set_content(row, col, content);
        }
        self.set_dirty();
    }
    fn cell_get_row_col(&self, row: usize, col: usize) -> String {
        if self.rows.is_valid_coords(row, col) {
            self.rows.get_owned(row, col).unwrap_or_default()
        } else {
            // should never happen
            String::new()
        }
    }
    fn cell_get_header(&self, col: usize) -> String {
        if col <= self.width() {
            self.headers.get(col).unwrap().to_owned()
        } else {
            String::new()
        }
    }

    pub fn append_column_named(&mut self, name: &str) {
        self.headers.push(String::from(name));
        self.rows.append_column();
    }
    fn set_column_name(&mut self, col: usize, content: String) {
        let value = self.headers.get_mut(col).unwrap();
        *value = content;
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }
    pub fn width(&self) -> usize {
        self.headers.len()
    }
    pub fn has_data(&self) -> bool {
        self.height() > 0 && self.width() > 0
    }
    pub fn header_widths(&self) -> Vec<u16> {
        self.headers
            .clone()
            .into_iter()
            .map(|h| h.len() as u16)
            .collect_vec()
    }
}
