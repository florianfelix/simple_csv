use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct CsvFileDescription {
    pub path: PathBuf,
    pub data: String,
    pub delim: char,
}

#[derive(Default, Debug, Clone)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone)]
pub struct CsvParseResult {
    pub errors: Vec<String>,
    pub csv_data: CsvData,
}
