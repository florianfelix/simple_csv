#![allow(unused)]
use std::path::PathBuf;

use text_buffer::Buffer;

pub trait RowsExt {
    fn append_column(&mut self);
    fn get_ref(&self, row: usize, column: usize) -> Option<&str>;
    fn get_owned(&self, row: usize, column: usize) -> Option<String>;
    fn set_content(&mut self, row: usize, col: usize, content: String);
    fn column_widths_min(&self, widths: Vec<u16>) -> Vec<u16>;
    fn is_valid_coords(&self, row: usize, col: usize) -> bool;
    fn get_column(&self, col: usize) -> Vec<String>;
}

impl RowsExt for Vec<Vec<String>> {
    fn append_column(&mut self) {
        for row in self.iter_mut() {
            row.push(String::new());
        }
    }
    fn get_ref(&self, row: usize, column: usize) -> Option<&str> {
        if let Some(r) = self.get(row) {
            if let Some(c) = r.get(column) {
                Some(c.as_str())
            } else {
                None
            }
        } else {
            None
        }
    }
    fn get_owned(&self, row: usize, column: usize) -> Option<String> {
        if let Some(r) = self.get(row) {
            r.get(column).map(|c| c.to_owned())
        } else {
            None
        }
    }
    fn set_content(&mut self, row: usize, col: usize, content: String) {
        let row = self.get_mut(row).expect("row index out of bounds");
        let value = row.get_mut(col).expect("out of bounds");
        *value = content;
    }
    fn column_widths_min(&self, mut widths: Vec<u16>) -> Vec<u16> {
        for row in self.iter() {
            for (column_index, content) in row.iter().enumerate() {
                if let Some(column_width) = widths.get_mut(column_index) {
                    if content.len() as u16 > *column_width {
                        *column_width = content.len() as u16;
                    }
                }
            }
        }
        widths.iter_mut().for_each(|w| *w += 1);
        widths
    }
    fn is_valid_coords(&self, row: usize, col: usize) -> bool {
        if row >= self.len() {
            false
        } else {
            let r = self.get(row).expect("Should never be out of bounds");
            col <= r.len()
        }
    }
    fn get_column(&self, col: usize) -> Vec<String> {
        self.iter()
            .map(|r| r.get(col).unwrap_or(&String::new()).to_owned())
            .collect::<Vec<String>>()
    }
}

pub trait BufferExt {
    fn to_cursor_string(&self) -> String;
}

impl BufferExt for Buffer {
    fn to_cursor_string(&self) -> String {
        let mut res = self.to_string();
        res.insert(self.cursor().chars(), '|');
        res
    }
}

impl BufferExt for Option<PathBuf> {
    fn to_cursor_string(&self) -> String {
        match self {
            None => String::new(),
            Some(path) => path.to_string_lossy().into_owned(),
        }
    }
}
