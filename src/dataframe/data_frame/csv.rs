use std::io::Write;
use std::path::PathBuf;

use csv::WriterBuilder;
use itertools::Itertools;

use super::{DataFrame, DataRow, DataType, DataValue, FrameError, FrameResult, Header};

#[derive(Default, Debug, Clone)]
pub struct DataFrameCsvResult {
    pub df: DataFrame,
    pub errors: Vec<String>,
}

impl DataFrame {
    pub fn parsed_from_csv(input: &str, delim: char) -> FrameResult<DataFrameCsvResult> {
        let input: &[u8] = input.as_bytes();
        let mut rdr = csv::ReaderBuilder::default()
            .delimiter(delim as u8)
            .trim(csv::Trim::All)
            .has_headers(true)
            .from_reader(input);

        let mut rows: Vec<DataRow> = vec![];
        let headers = rdr
            .headers()
            .map_err(|e| FrameError::Panic(format!("header not valid Utf8. {:}", e)))?;

        let mut headers = headers.iter().map(Header::new).collect_vec();

        let mut errors: Vec<String> = vec![];

        for (line_index, res) in rdr.deserialize::<DataRow>().enumerate() {
            match res {
                Ok(row) => {
                    headers.iter_mut().zip(row.iter()).for_each(|(h, v)| {
                        if h.dtype() == &DataType::Null {
                            h.set_dtype(v.into());
                        }
                    });
                    if row_matches_header(&row, &headers) {
                        rows.push(row)
                    } else {
                        errors.push(format!(
                            "Type error: line: {} {row:?} incompatible with header types {:}",
                            line_index + 2,
                            headers.iter().map(|h| h.to_debug()).join(",")
                        ));
                    }
                }
                Err(e) => {
                    errors.push(e.to_string());
                }
            }
        }
        Ok(DataFrameCsvResult {
            df: DataFrame::new(headers, rows).unwrap(),
            errors,
        })
    }

    pub fn to_csv(df: &DataFrame) -> FrameResult<String> {
        let mut wtr = WriterBuilder::new()
            .delimiter(",".as_bytes()[0])
            .from_writer(vec![]);
        wtr.write_record(df.headers.clone().iter().map(|h| h.name()).collect_vec())?;

        for row in df.rows.clone() {
            wtr.serialize(row)?;
        }
        Ok(String::from_utf8(
            wtr.into_inner()
                .map_err(|e| FrameError::Csv(e.to_string()))?,
        )?)
    }
}
fn row_matches_header(row: &[DataValue], headers: &Vec<Header>) -> bool {
    let r = row
        .iter()
        .zip(headers)
        .map(|(v, h)| &v.dtype() == h.dtype() || v.dtype() == DataType::Null)
        .all_equal();
    r
}

pub fn save_file(path: &PathBuf, content: &[u8]) -> FrameResult<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    file.write_all(content).unwrap();
    Ok(())
}
