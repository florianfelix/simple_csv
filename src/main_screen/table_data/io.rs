use itertools::Itertools;

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
        let record = res.unwrap();
        // let record = DataRow::from_iter(record);
        records.push(record);
    }
    (headers, records)
}
