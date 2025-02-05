use itertools::Itertools;
use tracing::error;

use crate::event::csv::{CsvData, CsvParseResult};

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
    let data = CsvData { headers, rows };

    CsvParseResult { errors, data }
}

pub fn headers_rows_from_csv_string(
    input: &str,
    delimiter: char,
) -> (Vec<String>, Vec<Vec<String>>) {
    let input: &[u8] = input.as_bytes();
    let mut rdr = csv::ReaderBuilder::default()
        .delimiter(delimiter as u8)
        .trim(csv::Trim::All)
        .has_headers(true)
        // .flexible(true)
        .from_reader(input);

    let mut records = vec![];
    let headers = rdr.headers().unwrap();
    let headers = headers.iter().map(|h| h.to_string()).collect_vec();

    for res in rdr.deserialize::<Vec<String>>() {
        match res {
            Ok(record) => records.push(record),
            Err(e) => {
                error!("{:#?}", e);
            }
        }
        // let record = res.unwrap();
        // let record = DataRow::from_iter(record);
        // records.push(record);
    }
    (headers, records)
}
