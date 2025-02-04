use itertools::Itertools;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

use crate::AppResult;

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

pub async fn load_csv(path: PathBuf, delim: char) -> CsvParseResult {
    let csv_str = path_to_string(&path).await.unwrap();
    parse_csv(&csv_str, delim)
}

async fn path_to_string(path: &PathBuf) -> AppResult<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}

pub fn parse_csv(input: &str, delimiter: char) -> CsvParseResult {
    let input: &[u8] = input.as_bytes();
    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(delimiter as u8)
        .trim(csv::Trim::All)
        .has_headers(true)
        // .flexible(true)
        .from_reader(input);

    let mut rows: Vec<Vec<String>> = vec![];
    let mut errors: Vec<String> = vec![];

    let headers = rdr
        .headers()
        .map_err(|e| errors.push(format!("{:#?}", e)))
        .unwrap();

    let headers = headers.iter().map(|h| h.to_string()).collect_vec();

    for res in rdr.deserialize::<Vec<String>>() {
        match res {
            Ok(record) => rows.push(record),
            Err(e) => {
                // error!("{:#?}", &e);
                errors.push(format!("{}", e));
            }
        }
    }
    let csv_data = CsvData { headers, rows };

    CsvParseResult { errors, csv_data }
}
