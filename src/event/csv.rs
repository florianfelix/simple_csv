use itertools::Itertools;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

use crate::AppResult;

use super::{ActionError, ActionResult};

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
    pub data: CsvData,
    pub path: Option<PathBuf>,
}

pub async fn load_csv(path: PathBuf, delim: char) -> ActionResult<CsvParseResult> {
    let res = path_to_string(&path).await;
    match res {
        Err(e) => Err(ActionError::FileIo {
            path,
            error: e.to_string(),
        }),
        Ok(res) => Ok(parse_csv(path, &res, delim)),
    }
}

pub fn parse_csv(path: PathBuf, input: &str, delimiter: char) -> CsvParseResult {
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

    // let errors = match errors.is_empty() {
    //     true => None,
    //     false => Some(errors),
    // };

    CsvParseResult {
        errors,
        data,
        path: Some(path),
    }
}

async fn path_to_string(path: &PathBuf) -> AppResult<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
