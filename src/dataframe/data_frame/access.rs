use super::{DataFrame, DataRow, DataType, DataValue, Header};
use itertools::Itertools;
use tracing::info;

impl DataFrame {
    pub fn headers(&self) -> &Vec<Header> {
        &self.headers
    }
    pub fn headers_mut(&mut self) -> &Vec<Header> {
        &mut self.headers
    }
    pub fn header_get(&self, col: usize) -> Option<&Header> {
        self.headers.get(col)
    }
    pub fn header_set(&mut self, col: usize, name: String) {
        if let Some(header) = self.headers.get_mut(col) {
            header.set_name(&name);
        }
    }
    pub fn rows(&self) -> &Vec<DataRow> {
        &self.rows
    }
    pub fn rows_mut(&mut self) -> &mut Vec<DataRow> {
        &mut self.rows
    }
    pub fn get(&self, row: usize, col: usize) -> Option<&DataValue> {
        if let Some(row) = self.rows.get(row) {
            if let Some(value) = row.get(col) {
                return Some(value);
            }
        }
        None
    }
    pub fn get_print(&self, row: usize, col: usize) -> String {
        if let Some(row) = self.rows.get(row) {
            if let Some(value) = row.get(col) {
                return value.to_string();
            }
        }
        info!("{:#?}", "NEW");
        String::new()
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut DataValue> {
        if let Some(row) = self.rows.get_mut(row) {
            if let Some(value) = row.get_mut(col) {
                return Some(value);
            }
        }
        None
    }
    pub fn parse_set(&mut self, row: usize, col: usize, value: &str) {
        if self.is_valid(row, col) {
            let dtype_col = self.dtype_column(col).expect("col to be in range");
            let value = dtype_col.parse(value);
            self.set(row, col, value);
        }
    }
    pub fn set(&mut self, row: usize, col: usize, value: impl Into<DataValue>) {
        let value: DataValue = value.into();
        info!("Setting DataValue {:?}", value);
        if self.is_valid(row, col) {
            let dtype_col = self.dtype_column(col).expect("col to be in range");

            if dtype_col == value.dtype() {
                let row = self.rows.get_mut(row).expect("row to be in range");
                let v = row.get_mut(col).expect("col to be in range");
                *v = value.clone();
            }
        }
    }
    pub fn column_get(&self, col: usize) -> Vec<&DataValue> {
        self.rows
            .iter()
            .filter_map(|row| row.get(col))
            .collect_vec()
    }
    pub fn column_get_print(&self, col: usize) -> Vec<String> {
        self.rows
            .iter()
            .filter_map(|row| row.get(col))
            .map(|e| e.print())
            .collect_vec()
    }
    pub fn dtype_column(&self, col: usize) -> Option<DataType> {
        if self.is_valid_col(col) {
            Some(
                self.headers
                    .get(col)
                    .expect("col to be in range")
                    .dtype()
                    .clone(),
            )
        } else {
            None
        }
    }
    pub fn append_empty_row(&mut self) {
        self.rows.push(DataRow::new(self.width()));
    }
    pub fn append_empty_column(&mut self, dtype: DataType) {
        let header = Header::new("new column").with_dtype(dtype);
        info!("{:#?}", &header);
        self.headers.push(header);
        self.rows.iter_mut().for_each(|r| r.push(DataValue::Null));
    }
    pub fn append_empty_column_named(&mut self, dtype: DataType, name: &str) {
        let header = Header::new(name).with_dtype(dtype);
        self.headers.push(header);
        self.rows.iter_mut().for_each(|r| r.push(DataValue::Null));
    }
    pub fn remove_row(&mut self, row: usize) {
        if self.is_valid_row(row) {
            self.rows.remove(row);
        }
    }
    pub fn remove_column(&mut self, col: usize) {
        if self.is_valid_col(col) {
            self.headers.remove(col);
            self.rows.iter_mut().for_each(|row| {
                row.remove(col);
            });
        }
    }
    pub fn move_row_up(&mut self, row: usize) -> Option<usize> {
        if self.is_valid_row(row) && row > 0 {
            self.rows.swap(row, row - 1);
            Some(row - 1)
        } else {
            None
        }
    }
    pub fn move_row_down(&mut self, row: usize) -> Option<usize> {
        if self.is_valid_row(row) && row < self.rows.len() - 1 {
            self.rows.swap(row, row + 1);
            Some(row + 1)
        } else {
            None
        }
    }
    pub fn move_column_left(&mut self, col: usize) -> Option<usize> {
        if self.is_valid_col(col) && col > 0 {
            self.headers.swap(col, col - 1);
            self.rows.iter_mut().for_each(|row| {
                row.swap(col, col - 1);
            });
            return Some(col - 1);
        }
        None
    }
    pub fn move_column_right(&mut self, col: usize) -> Option<usize> {
        if self.is_valid_col(col) && col < self.width() - 1 {
            self.headers.swap(col, col + 1);
            self.rows.iter_mut().for_each(|row| {
                row.swap(col, col + 1);
            });
            return Some(col + 1);
        }
        None
    }
    pub fn min_header_widths(&self) -> Vec<u16> {
        self.headers
            .iter()
            .map(|h| h.name().len() as u16)
            .collect_vec()
    }
    pub fn min_column_widths(&self) -> Vec<u16> {
        let mut widths = self.min_header_widths();
        for row in &self.rows {
            for (column_index, value) in row.iter().enumerate() {
                if widths.get(column_index).is_some() {
                    let width = value.print().len() as u16;
                    if width > widths[column_index] {
                        widths[column_index] = width;
                    }
                }
            }
        }
        widths
    }
    pub fn column_sort(&mut self, col: usize) {
        if self.is_valid_col(col) {
            self.rows.sort_by(|a, b| {
                a.get(col)
                    .expect("valid column index")
                    .cmp(b.get(col).expect("valid column index"))
            });
        }
    }
    pub fn column_sort_desc(&mut self, col: usize) {
        if self.is_valid_col(col) {
            self.rows.sort_by(|a, b| {
                b.get(col)
                    .expect("valid column index")
                    .cmp(a.get(col).expect("valid column index"))
            });
        }
    }
    pub fn column_set_dtype(&mut self, col: usize, dtype: DataType) {
        if self.is_valid_col(col) {
            self.headers[col].set_dtype(dtype.clone());
            self.rows.iter_mut().for_each(|row| {
                if let Some(value) = row.get_mut(col) {
                    value.convert_dtype(dtype.clone());
                }
            });
        }
    }
}
