extern crate filing_parser;

use aws::s3;
use filing_parser::helpers::{path_exists, write_to_file};
use filing_parser::test_files::get_files;
use flate2::read::GzDecoder;
use std::io::prelude::*;
use tendril::fmt::Slice;

/// Downloads any filings that are missing from the examples needed for testing
fn main() {
    let client = s3::get_s3_client();
    let files = get_files();
    for file in files {
        let path = String::from(file.path);
        if !path_exists(&path) {
            let data = s3::get_s3_object(&client, "birb-edgar-filings", file.s3);
            let mut d = GzDecoder::new(data.as_bytes());
            let mut decompressed = String::new();
            d.read_to_string(&mut decompressed).unwrap();

            println!("{}", data.len());
            write_to_file(&path, ".txt", decompressed.clone().into_bytes())
                .expect("Couldn't write to file!");
            write_to_file(&path, ".html", decompressed.clone().into_bytes())
                .expect("Couldn't write to file!");
        }
    }
}
