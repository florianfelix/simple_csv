use dataframe::DataFrame;
use std::path::PathBuf;

use crate::backend::{utils::read_file, IoCommandError, IoCommandResult};

#[derive(Default, Debug, Clone)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone)]
pub struct CsvDescription {
    pub df: DataFrame,
    pub errors: Vec<String>,
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
            let data_frame_csv_result = DataFrame::parsed_from_csv(&res, delim)?;

            Ok(CsvDescription {
                path: Some(path),
                delim,
                df: data_frame_csv_result.df,
                errors: data_frame_csv_result.errors,
            })
        }
    }
}

// impl CsvDescription {
//     pub fn data_to_string(&self) -> AppResult<String> {
//         let mut wtr = WriterBuilder::new()
//             .delimiter(self.delim as u8)
//             .from_writer(vec![]);
//         wtr.write_record(self.data.headers.clone())?;

//         for row in self.data.rows.clone() {
//             wtr.write_record(row)?;
//         }
//         let data = String::from_utf8(wtr.into_inner()?)?;
//         Ok(data)
//     }
// }
