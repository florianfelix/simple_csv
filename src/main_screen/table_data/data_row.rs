use indexmap::IndexMap;
use itertools::Itertools;
use ratatui::{layout::Constraint, style::Style, widgets};
use tracing::info;

use super::data_cell::DataCell;

#[derive(Default, Debug, Clone)]
pub struct DataRow {
    fields: IndexMap<String, DataCell>,
    style: Style,
}

impl DataRow {
    pub fn headers(&self) -> Vec<String> {
        self.fields.keys().cloned().collect_vec()
    }
    pub fn rat_row(&self) -> widgets::Row {
        widgets::Row::new(self.rat_cells()).style(self.style)
    }
    fn rat_cells(&self) -> Vec<widgets::Cell> {
        self.fields
            .iter()
            .map(|d| widgets::Cell::new(d.1.to_string()))
            .collect_vec()
    }
    pub fn len(&self) -> usize {
        self.fields.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    // fn equal_widths(&self) -> Vec<Constraint> {
    //     let num_col = self.fields.len();

    // }
}

impl DataRow {
    pub fn set_col(&mut self, column: &str, value: &str) {
        if self.fields.contains_key(column) {
            let cell = DataCell::from_tuple((column.to_string(), value.to_string()));
            self.fields.insert(column.to_string(), cell);
        }
    }
    pub fn set_idx(&mut self, idx: usize, value: &str) {
        if idx <= self.len() {
            let cell = self.fields.get_index_mut(idx).unwrap().1;
            cell.set(value);
        }
    }
    pub fn get_idx(&self, idx: usize) -> String {
        self.fields.get_index(idx).unwrap().1.to_string()
    }
}

impl DataRow {
    fn from_iter(value: impl IntoIterator<Item = (String, String)>) -> Self {
        let fields: Vec<(String, DataCell)> = value
            .into_iter()
            .map(|i| (i.0.clone(), DataCell::from_tuple(i)))
            .collect_vec();
        let fields: IndexMap<String, DataCell> = IndexMap::from_iter(fields);
        DataRow {
            fields,
            style: Style::default(),
        }
    }
    pub fn from_csv_string(input: &str, delimiter: char) -> Vec<DataRow> {
        let input: &[u8] = input.as_bytes();
        let mut rdr = csv::ReaderBuilder::default()
            .delimiter(delimiter as u8)
            .trim(csv::Trim::All)
            .has_headers(true)
            // .flexible(true)
            .from_reader(input);

        let mut records = vec![];
        for res in rdr.deserialize::<IndexMap<String, String>>() {
            let record = res.unwrap();
            let record = DataRow::from_iter(record);
            records.push(record);
        }
        records
    }
}

impl DataRow {
    pub fn examples() -> Vec<DataRow> {
        let input = include_str!("sample.csv");
        let s = Self::from_csv_string(input, ';');
        info!("{:#?}", &s);
        s
    }
}
