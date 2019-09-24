use aws::s3::{get_s3_client, get_s3_object, store_s3_document_gzipped};
use filing_parser::split_full_submission::split_full_submission;
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::Connection;
use rusoto_s3::S3Client;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

/// Do the main work infinitely
fn main() {
    //    loop {
    _main();
    //    }
}

/// Finds a filing that has been collected, but not yet split.
///     1. Splits it
///     2. Compresses its parts
///     3. Uploads them to S3
///     4. Inserts records into `split_filings` to indicate that they were uploaded
fn _main() {
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let conn = get_connection(db_uri);
    let s3_client = get_s3_client();

    // Collect:
    let (filing, filing_s3_path, filing_id) =
        collect_random_not_yet_split_filing(&conn, &s3_client);

    // Split:
    let split_filings = split_full_submission(&*filing);

    let cik = get_cik(&*filing_s3_path);
    let accession_number = get_accession_number(&*filing_s3_path);
    let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);

    upload_to_s3(&s3_client, &split_filings, &s3_url_prefix);
    persist_split_filing_to_db(&conn, &split_filings, &s3_url_prefix, &filing_id);
}

fn collect_random_not_yet_split_filing(
    conn: &Connection,
    s3_client: &S3Client,
) -> (String, String, i32) {
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
    let rows = &conn.query(query, &[]).expect("");
    assert_eq!(
        1,
        rows.len(),
        "Should have received one row! Instead received {} rows",
        rows.len()
    );
    let filing = Filing::from_row(rows.get(0));
    let s3_path = format!("{}.gz", filing.filing_edgar_url);
    let object_contents = get_s3_object(&s3_client, "birb-edgar-filings", &*s3_path);
    (
        decompress_gzip(object_contents),
        filing.filing_edgar_url,
        filing.id,
    )
}

fn upload_to_s3(
    s3_client: &S3Client,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
    s3_url_prefix: &String,
) {
    for doc in split_filings {
        let contents_for_file = match &doc.decoded_text {
            Some(d) => d.clone(),
            None => doc.text.clone().into_bytes(),
        };
        let path = format!("{}/{}", s3_url_prefix, doc.filename);
        store_s3_document_gzipped(s3_client, "birb-edgar-filings", &*path, contents_for_file)
            .expect("Couldn't upload document!");
        println!("File path: {}", path);
    }
}

fn persist_split_filing_to_db(
    conn: &Connection,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
    s3_url_prefix: &String,
    filing_id: &i32,
) {
    let trans = conn.transaction().expect("Couldn't begin transaction");

    let statement = trans
        .prepare(
            "
             INSERT INTO split_filing
             (filing_id, sequence, doc_type, filename, description, s3_url_prefix)
             VALUES ($1, $2, $3, $4, $5, $6);
             ",
        )
        .expect("Couldn't prepare company upsert statement for execution");

    for each in split_filings {
        statement
            .execute(&[
                filing_id,
                &each.sequence,
                &each.doc_type,
                &each.filename,
                &each.description,
                s3_url_prefix,
            ])
            .expect("Couldn't execute update");
    }

    trans.commit().expect("Couldn't insert into split_filing")
}
