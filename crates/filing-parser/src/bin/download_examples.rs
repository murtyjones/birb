extern crate filing_parser;

use aws::s3;
use filing_parser::helpers::{get_abs_path, path_exists};
use filing_parser::test_files::FILES;
use std::fs::{metadata, File};
use std::io::prelude::*;

fn write_to_file(file_path: &String, data: Vec<u8>) -> std::io::Result<()> {
    let absolute_path = get_abs_path(file_path);
    let mut pos = 0;
    let mut buffer = File::create(absolute_path).expect("Couldn't make file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

/// Downloads any filings that are missing from the examples needed for testing
fn main() {
    let client = s3::get_s3_client();
    for i in 0..FILES.len() {
        let file = &FILES[i];
        if !path_exists(&file.path) {
            let data = s3::get_s3_object(&client, "birb-edgar-filings", file.s3.as_str());
            println!("{}", data.len());
            write_to_file(&file.path, data);
        }
    }
}
