use csv::WriterBuilder;
use itertools::Itertools;
use std::path::PathBuf;

use crate::{
    backend::{utils::read_file, IoCommandError, IoCommandResult},
    AppResult,
};

#[derive(Default, Debug, Clone)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone)]
pub struct CsvDescription {
    pub errors: Vec<String>,
    pub data: CsvData,
    pub path: Option<PathBuf>,
    pub delim: char,
}

pub async fn load_csv(path: PathBuf, delim: char) -> IoCommandResult<CsvDescription> {
    let res = read_file(&path).await;
    match res {
        Err(e) => Err(IoCommandError::FileIo {
            path,
            error: e.to_string(),
        }),
        Ok(res) => {
            let input: &[u8] = res.as_bytes();
            let mut rdr = csv::ReaderBuilder::default()
                .delimiter(delim as u8)
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

            Ok(CsvDescription {
                errors,
                data,
                path: Some(path),
                delim,
            })
        }
    }
}

impl CsvDescription {
    pub fn data_to_string(&self) -> AppResult<String> {
        let mut wtr = WriterBuilder::new()
            .delimiter(self.delim as u8)
            .from_writer(vec![]);
        wtr.write_record(self.data.headers.clone())?;

        for row in self.data.rows.clone() {
            wtr.write_record(row)?;
        }
        let data = String::from_utf8(wtr.into_inner()?)?;
        Ok(data)
    }
}
