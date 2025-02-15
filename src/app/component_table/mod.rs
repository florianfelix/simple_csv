use itertools::Itertools;
use ratatui::widgets::TableState;
use std::path::PathBuf;

mod actions;
mod actions_cell_select;
mod commands;
mod extensions;
mod popups;
mod render;
mod skim;

use skim::Skim;
use text_buffer::Buffer;
#[allow(unused)]
use tracing::info;

use crate::backend::file_formats::file_csv::CsvDescription;
use extensions::RowsExt;

#[derive(Default, Debug, Clone)]
pub enum EditTarget {
    #[default]
    None,
    Cell((usize, usize)),
    Header(usize),
    FileName,
}

#[derive(Debug)]
pub struct DataTable {
    pub(crate) headers: Vec<String>,
    pub(crate) rows: Vec<Vec<String>>,
    pub table_state: TableState,
    pub textbuffer: text_buffer::Buffer,
    pub edit_target: EditTarget,
    pub skim: Option<Skim>,
    pub path: Option<PathBuf>,
    pub delim: char,
    pub is_dirty: bool,
    pub parse_errors: Vec<String>,
}

impl Default for DataTable {
    fn default() -> Self {
        Self {
            headers: vec![String::from("key"), String::from("value")],
            rows: vec![vec![String::from(""), String::from("")]],
            table_state: TableState::default(),
            textbuffer: Buffer::new(),
            edit_target: EditTarget::None,
            skim: None,
            path: None,
            delim: ';',
            is_dirty: true,
            parse_errors: vec![],
        }
    }
}

impl DataTable {
    pub fn from_csv_description(&mut self, csv_description: CsvDescription) {
        self.headers = csv_description.data.headers;
        self.rows = csv_description.data.rows;
        self.table_state = TableState::default();
        self.textbuffer = Buffer::new();
        self.edit_target = EditTarget::None;
        self.path = csv_description.path;
        self.delim = csv_description.delim;
        self.is_dirty = false;
        self.parse_errors = csv_description.errors;
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
    fn header_widths(&self) -> Vec<u16> {
        self.headers
            .clone()
            .into_iter()
            .map(|h| h.len() as u16)
            .collect_vec()
    }
}
