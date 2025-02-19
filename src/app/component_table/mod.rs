use dataframe::DataFrame;
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
    pub df: DataFrame,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
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
            df: DataFrame::default(),
            headers: vec![String::from("key"), String::from("value")],
            rows: vec![vec![String::from(""), String::from("")]],
            table_state: TableState::default(),
            textbuffer: Buffer::new(),
            edit_target: EditTarget::None,
            skim: None,
            path: Some(PathBuf::from("new.csv")),
            delim: ';',
            is_dirty: true,
            parse_errors: vec![],
        }
    }
}

#[allow(clippy::field_reassign_with_default)]
impl From<CsvDescription> for DataTable {
    fn from(csv_description: CsvDescription) -> Self {
        let mut data_table = DataTable::default();

        data_table.df = csv_description.df;
        data_table.parse_errors = csv_description.errors;

        // data_table.headers = csv_description.data.headers;
        // data_table.rows = csv_description.data.rows;
        data_table.path = csv_description.path;
        data_table.delim = csv_description.delim;
        data_table
    }
}
