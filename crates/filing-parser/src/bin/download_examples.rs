extern crate filing_parser;

use aws::s3;
use filing_parser::helpers::{path_exists, write_to_file};
use filing_parser::test_files::get_files;

/// Downloads any filings that are missing from the examples needed for testing
fn main() {
    let client = s3::get_s3_client();
    let files = get_files();
    for file in files {
        let path = String::from(file.path);
        if !path_exists(&path) {
            let data = s3::get_s3_object(&client, "birb-edgar-filings", file.s3);
            println!("{}", data.len());
            write_to_file(&path, data).expect("Couldn't write to file!");
        }
    }
}
