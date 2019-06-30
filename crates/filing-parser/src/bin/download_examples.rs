extern crate filing_parser;

use aws::s3;
use filing_parser::helpers::{get_abs_path, path_exists, write_to_file};
use filing_parser::test_files::FILES;

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
