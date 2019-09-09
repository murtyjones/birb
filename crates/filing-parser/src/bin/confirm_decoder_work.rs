extern crate filing_parser;

use aws::s3::{get_s3_client, get_s3_object};
use filing_parser::split_full_submission::split_full_submission;
use models::{Filing, SplitDocumentBeforeUpload};
use utils::{decompress_gzip, delete_dir_contents, get_connection, write_to_file};

const EXAMPLE_PARSED_FOLDER_PATH: &'static str = "/Users/murtyjones/Desktop/example-parsed";

/// Confirm the decoder works by processing a bunch of random filings
/// locally and writing to example-parsed. This file is a throwaway.
fn main() {
    // Clear the contents of the "example-parsed" folder
    delete_dir_contents(EXAMPLE_PARSED_FOLDER_PATH);
    loop {
        _main();
    }
}

fn _main() {
    // Get local DB connection:
    let local_connection = get_connection("postgres://postgres:develop@localhost:5432/postgres");

    // Get an S3 client
    let s3_client = get_s3_client();

    // Get a random filing where the filing has
    // been collected, but has NOT been split yet:
    let query = r"
        SELECT f.* FROM filing f
        LEFT JOIN split_filing sf on f.id = sf.filing_id
        WHERE f.collected = true
        AND sf.filing_id IS NULL
        ORDER BY random()
        LIMIT 1
        ;
    ";

    // Execute query
    let rows = &local_connection.query(query, &[]).expect("");
    assert_eq!(
        1,
        rows.len(),
        "Should have received one row! Instead received {} rows",
        rows.len()
    );
    let filing = Filing::from_row(rows.get(0));
    let s3_path = format!("{}.gz", filing.filing_edgar_url);
    let object_contents = get_s3_object(&s3_client, "birb-edgar-filings", &*s3_path);
    let decompressed = decompress_gzip(object_contents);
    let parsed_docs = split_full_submission(&*decompressed);
    write_parsed_docs_to_example_folder(parsed_docs);
}

/// Converts docs' contents to bytes (which may include uudecoding)
/// and writes them to an example folder locally.
fn write_parsed_docs_to_example_folder(parsed_documents: Vec<SplitDocumentBeforeUpload>) {
    for doc in parsed_documents {
        let mut contents_for_file = doc.text.as_bytes().to_owned();
        if doc.doc_type == "GRAPHIC" {
            //            panic!("{:?}", doc.text);
            contents_for_file = uuencode::uudecode(&*doc.text)
                .expect("Couldn't uudecode document contents!")
                .0;
        }
        write_to_file(
            &String::from(format!("{}/{}", EXAMPLE_PARSED_FOLDER_PATH, doc.filename,)),
            "",
            contents_for_file,
        )
        .expect("Couldn't write to file!");
    }
}
