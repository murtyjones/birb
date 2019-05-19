extern crate csv;
extern crate serde;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::io;
use std::process;
use std::vec::Vec;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
pub struct FilingMetdata {
    short_cik: String,
    company_name: String,
    form_type: String,
    date_filed: String,
    filename: String,
}

/// Deserializes an index
pub fn main(data: &'static str) -> Result<Vec<FilingMetdata>, Box<Error>> {
    let trimmed_data = trim_index(data);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_reader(trimmed_data.as_bytes());
    let mut filing_metadatas = vec![];
    for (i, result) in rdr.deserialize().enumerate() {
        // We can and should safely skip the first 11 lines of any
        // index file as they don't contain the data we are about.

        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: FilingMetdata = result?;
        filing_metadatas.push(record);
    }
    Ok(filing_metadatas)
}

/// Trims the first lines of an edgar index.
fn trim_index(data: &'static str) -> String {
    let mut trimmed_data = String::from("");
    for (i, line) in data.lines().enumerate() {
        if i <= 10 {
            continue;
        }
        trimmed_data.push_str(line);
        trimmed_data.push_str("\n");
    }
    trimmed_data
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_main() {
        let example_index = include_str!("../../../data/edgar-indexes/2016/QTR1/master.idx");
        let mut filing_metadatas = main(example_index).expect("Couldn't parse index");
        // total lines in file: 308910
        // ignored lines: 11
        // total lines - ignored lines = 308,899
        assert_eq!(308_899, filing_metadatas.len());
        let final_item = filing_metadatas
            .pop()
            .expect("Couldn't unwrap last filer in collection");
        assert_eq!(final_item.short_cik, String::from("99947"));
        assert_eq!(
            final_item.company_name,
            String::from("TRUBEE, COLLINS & CO., INC.")
        );
        assert_eq!(final_item.form_type, String::from("X-17A-5"));
        assert_eq!(final_item.date_filed, String::from("2016-02-24"));
        assert_eq!(
            final_item.filename,
            String::from("edgar/data/99947/9999999997-16-017437.txt")
        );
    }
}
