use aws::s3::{get_s3_client, store_s3_document_gzipped_async};
use failure;
use filing_parser::split_full_submission::split_full_submission;
use futures::{future, Future, Stream};
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::Connection;
use rayon::prelude::*;
use rusoto_core::RusotoFuture;
use rusoto_s3::S3Client;
use rusoto_s3::S3;
use rusoto_s3::{GetObjectOutput, GetObjectRequest};
use rusoto_s3::{PutObjectError, PutObjectOutput};
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let conn = get_connection(db_uri);
    let s3_client = get_s3_client();

    let now = std::time::Instant::now();
    let filings = collect_random_not_yet_split_filings(&conn, &s3_client);
    println!("Took {:?} to query for un-split filings", now.elapsed());

    let now = std::time::Instant::now();
    let split_filings = filings
        .par_iter()
        .map(|(filing, filing_s3_path, filing_id)| {
            println!("{}", filing_s3_path);
            (
                filing_s3_path,
                filing_id,
                split_full_submission(&*filing, &filing_id),
            )
        })
        .collect::<Vec<(&String, &i32, Vec<SplitDocumentBeforeUpload>)>>();
    println!("Took {:?} to split filings", now.elapsed());

    assert_eq!(
        filings.len(),
        split_filings.len(),
        "Expected equal number of filings and split_filings"
    );

    let now = std::time::Instant::now();
    upload_all(&s3_client, &split_filings).expect("Couldn't upload filings!");
    println!("Took {:?} to upload filings", now.elapsed());

    let now = std::time::Instant::now();
    persist_split_filings_to_db(&conn, &split_filings)
        .expect("Couldn't persist split filings to DB!");
    println!("Took {:?} to persist split filings to DB", now.elapsed());

    println!("Finished in {:?}", start.elapsed());
}

fn collect_random_not_yet_split_filings(
    conn: &Connection,
    s3_client: &S3Client,
) -> Vec<(String, String, i32)> {
    let mut core = Core::new().unwrap();
    let query = r"
        SELECT f.* FROM filing f
        LEFT JOIN split_filing sf on f.id = sf.filing_id
        WHERE f.collected = true
        AND sf.filing_id IS NULL
        ORDER BY random()
        LIMIT 10
        ;
    ";
    //    let query = r"select * from filing where filing_edgar_url = 'edgar/data/40545/0000040545-16-000152.txt'";
    // Execute query
    let rows = &conn.query(query, &[]).expect("");
    let promises = future::join_all(rows.iter().map(|row| {
        let filing = Filing::from_row(rows.get(0));
        let s3_path = format!("{}.gz", filing.filing_edgar_url);
        let get_req = GetObjectRequest {
            bucket: String::from("birb-edgar-filings"),
            key: s3_path,
            ..Default::default()
        };
        s3_client.get_object(get_req)
    }));

    let objects: Vec<GetObjectOutput> = match core.run(promises) {
        Ok(items) => items,
        Err(e) => panic!("Error completing futures: {}", e),
    };
    let objects = objects
        .into_iter()
        .map(|result| {
            let stream = result.body.unwrap();
            let body = stream.concat2().wait().unwrap();
            body.to_vec()
        })
        .collect::<Vec<Vec<u8>>>();

    let mut results: Vec<(String, String, i32)> = vec![];

    assert_eq!(rows.len(), objects.len());

    for (i, object_contents) in objects.into_iter().enumerate() {
        let row = rows.get(i);
        let filing = Filing::from_row(row);
        results.push((
            decompress_gzip(object_contents),
            filing.filing_edgar_url,
            filing.id,
        ));
    }

    results
}

fn upload_all(
    s3_client: &S3Client,
    split_filings: &Vec<(&String, &i32, Vec<SplitDocumentBeforeUpload>)>,
) -> Result<(), failure::Error> {
    let mut core = Core::new().unwrap();

    let mut flattened = vec![];

    for (filing_s3_path, filing_id, docs) in split_filings {
        for split in docs {
            flattened.push((filing_s3_path, filing_id, split));
        }
    }

    let promises = future::join_all(flattened.iter().map(|(filing_s3_path, _filing_id, doc)| {
        let cik = get_cik(&*filing_s3_path);
        let accession_number = get_accession_number(&*filing_s3_path);
        let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
        let contents_for_file = match &doc.decoded_text {
            Some(d) => d.clone(),
            None => doc.text.clone().into_bytes(),
        };
        let path = format!("{}/{}", s3_url_prefix, doc.filename);
        // The first document is always the important one, and therefore the
        // one that we want to restrict read access.
        let acl = match doc.sequence {
            1 => "private",
            _ => "public-read",
        };
        store_s3_document_gzipped_async(
            s3_client,
            "birb-edgar-filings",
            &*path,
            contents_for_file,
            acl,
        )
    }));

    match core.run(promises) {
        Ok(items) => println!("Uploaded!"),
        Err(e) => panic!("Error completing futures: {}", e),
    };

    Ok(())
}

fn persist_split_filings_to_db(
    conn: &Connection,
    split_filings: &Vec<(&String, &i32, Vec<SplitDocumentBeforeUpload>)>,
) -> Result<(), failure::Error> {
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

    for (filing_s3_path, filing_id, docs) in split_filings {
        for each in docs {
            let cik = get_cik(&*filing_s3_path);
            let accession_number = get_accession_number(&*filing_s3_path);
            let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
            statement
                .execute(&[
                    filing_id,
                    &each.sequence,
                    &each.doc_type,
                    &each.filename,
                    &each.description,
                    &s3_url_prefix,
                ])
                .expect("Couldn't execute update");
        }
    }
    trans.commit().expect("Couldn't insert into split_filing");
    Ok(())
}
