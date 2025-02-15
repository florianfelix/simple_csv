use text_buffer::Buffer;

use super::{extensions::RowsExt, skim::Skim, DataTable, EditTarget};

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
    }
    pub fn append_row(&mut self) {
        self.rows.push(vec![String::new(); self.width()]);
    }
    pub fn append_column(&mut self) {
        self.headers.push(String::from("NewColumn"));
        self.rows.append_column();
    }
}
