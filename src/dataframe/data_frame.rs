use serde::{Deserialize, Serialize};

mod access;
pub mod csv;
mod data_row;
mod data_type;
mod error;
mod export;
mod header;
mod value;
pub use data_row::DataRow;
pub use data_type::{DataType, DataTypeParseError};
pub use error::{FrameError, FrameResult};
pub use header::Header;
pub use value::{DataValue, Float};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    headers: Vec<Header>,
    rows: Vec<DataRow>,
}

impl Default for DataFrame {
    fn default() -> Self {
        Self {
            headers: vec![
                Header::new("column0").with_dtype(DataType::String),
                Header::new("column1").with_dtype(DataType::String),
            ],
            rows: vec![DataRow::new(2)],
        }
    }
}

impl DataFrame {
    pub fn new(headers: Vec<Header>, rows: Vec<DataRow>) -> FrameResult<Self> {
        let width = headers.len();
        if rows.iter().all(|row| row.len() == width) {
            Ok(Self { headers, rows })
        } else {
            Err(FrameError::NotUniformColumnWidths)
        }
    }
    pub fn width(&self) -> usize {
        self.headers.len()
    }
    pub fn height(&self) -> usize {
        self.rows.len()
    }
    pub fn is_valid(&self, row: usize, col: usize) -> bool {
        self.is_valid_row(row) && self.is_valid_col(col)
    }
    pub fn is_valid_row(&self, row: usize) -> bool {
        self.rows.len() > row
    }
    pub fn is_valid_col(&self, col: usize) -> bool {
        self.headers.len() > col
    }
}
