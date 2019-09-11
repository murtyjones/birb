extern crate filing_parser;
extern crate utils;

use aws::s3;
use filing_parser::helpers::path_exists;
use filing_parser::test_files::get_files;
use flate2::read::GzDecoder;
use tendril::fmt::Slice;
use utils::{decompress_gzip, write_to_file};

/// Downloads any filings that are missing from the examples needed for testing
fn main() {
    let client = s3::get_s3_client();
    let files = get_files();
    for file in files {
        let path = String::from(file.path);
        if !path_exists(&path) {
            let data = s3::get_s3_object(&client, "birb-edgar-filings", file.s3);
            let decompressed_data = decompress_gzip(data);

            write_to_file(&path, ".txt", decompressed_data.clone().into_bytes())
                .expect("Couldn't write to file!");
            write_to_file(&path, ".html", decompressed_data.clone().into_bytes())
                .expect("Couldn't write to file!");
        }
    }
}
