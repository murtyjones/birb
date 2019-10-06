use aws::s3::{get_s3_client, get_s3_object, store_s3_document_gzipped_async};
use chrono::Utc;
use failure;
use filing_parser::split_full_submission::split_full_submission;
use futures::future;
use models::{Filing, SplitDocumentBeforeUpload};
use postgres::rows::Rows;
use postgres::Connection;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use rusoto_core::RusotoFuture;
use rusoto_s3::S3Client;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio_core::reactor::Core;
use utils::{decompress_gzip, get_accession_number, get_cik, get_connection};

const NUMBER_TO_COLLECT_PER_ITERATION: i32 = 20;

use r2d2;
use r2d2::{Pool, PooledConnection};

fn main() {
    let start = std::time::Instant::now();
    let db_uri = std::env::var("DATABASE_URI").expect("No connection string found!");
    let manager = PostgresConnectionManager::new(db_uri, TlsMode::None).unwrap();
    let pool = Pool::new(manager).unwrap();
    let s3_client = get_s3_client();
    let num_threads = num_cpus::get();
    let mut filings = get_unsplit_filings(pool.get().unwrap())
        .into_iter()
        .map(|row| row.into())
        .collect::<Vec<Filing>>();

    let mut filings_to_process: Vec<Vec<Filing>> =
        get_filings_for_threads(&mut filings, num_threads);

    let mut handles = vec![];
    for _i in 0..num_threads {
        let pool = pool.clone();
        handles.push(spawn_worker(
            pool.get().unwrap(),
            filings_to_process.pop().unwrap(),
        ));
    }

    // wait for all threads to resolve
    for h in handles {
        h.join().unwrap();
    }

    println!("Took: {:?}", start.elapsed());
}

fn spawn_worker(
    conn: PooledConnection<PostgresConnectionManager>,
    queue: Vec<Filing>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        queue.into_iter().for_each(|filing| {
            let s3_client = get_s3_client();
            let s3_path = format!("{}.gz", filing.filing_edgar_url);
            let object_contents =
                decompress_gzip(get_s3_object(&s3_client, "birb-edgar-filings", &*s3_path));
            let split_filings = split_full_submission(&*object_contents, &filing.id);
            let cik = get_cik(&*filing.filing_edgar_url);
            let accession_number = get_accession_number(&*filing.filing_edgar_url);
            let s3_url_prefix = format!("edgar/data/{}/{}", cik, accession_number);
            upload_all(&s3_client, &filing, &split_filings).expect("Couldn't persist to DB!");
            persist_split_filing_to_db(&conn, &split_filings, &s3_url_prefix, &filing.id);
        });
    })
}

fn upload_all(
    s3_client: &S3Client,
    filing: &Filing,
    split_filings: &Vec<SplitDocumentBeforeUpload>,
) -> Result<(), failure::Error> {
    let mut core = Core::new().unwrap();
    let promises = future::join_all(split_filings.iter().map(|doc| {
        let cik = get_cik(&*filing.filing_edgar_url);
        let accession_number = get_accession_number(&*filing.filing_edgar_url);
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
            &s3_client,
            "birb-edgar-filings",
            &*path,
            contents_for_file,
            acl,
        )
    }));

    match core.run(promises) {
        Ok(_items) => println!("Uploaded!"),
        Err(e) => panic!("Error completing futures: {}", e),
    };

    Ok(())
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

fn get_filings_for_threads<T>(f: &mut Vec<T>, n: usize) -> Vec<Vec<T>>
where
    T: Clone + std::fmt::Debug,
{
    let mut i = 0;
    let mut chunks: Vec<Vec<T>> = vec![];
    for i in 0..n {
        let s: Vec<T> = Vec::new();
        chunks.push(s);
    }

    while f.len() > 0 {
        let item_to_insert = f.pop().expect("Item doesn't exist in vec!");
        let chunk: &mut Vec<T> = chunks.get_mut(i).expect("chunk doesn't exist");
        chunk.push(item_to_insert);
        i = (i + 1) % (n);
    }
    chunks
}

fn get_unsplit_filings(conn: PooledConnection<PostgresConnectionManager>) -> Rows {
    let query = format!(
        "
        SELECT * FROM filing f TABLESAMPLE SYSTEM ((100000 * 100) / 5100000.0)
        LEFT JOIN split_filing sf on f.id = sf.filing_id
        WHERE f.collected = true
        AND sf.filing_id IS NULL
        ORDER BY random()
        LIMIT {}
        ;
    ",
        NUMBER_TO_COLLECT_PER_ITERATION
    );
    //    let query = r"select * from filing where filing_edgar_url = 'edgar/data/40545/0000040545-16-000152.txt'";
    // Execute query
    conn.query(&*query, &[]).expect("Couldn't get rows!")
}

mod test {
    use super::*;

    #[test]
    fn test_chunk_splitting() {
        let mut input = vec![1, 2, 3, 4, 5, 6];
        let expected_output = vec![vec![6, 3], vec![5, 2], vec![4, 1]];
        let result = get_filings_for_threads(&mut input, 3);
        assert_eq!(expected_output, result);

        let mut input = vec![1, 2, 3, 4, 5, 6, 7];
        let expected_output = vec![vec![7, 4, 1], vec![6, 3], vec![5, 2]];
        let result = get_filings_for_threads(&mut input, 3);
        assert_eq!(expected_output, result);

        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let expected_output = vec![
            vec![14, 11, 8, 5, 2],
            vec![13, 10, 7, 4, 1],
            vec![12, 9, 6, 3],
        ];
        let result = get_filings_for_threads(&mut input, 3);
        assert_eq!(expected_output, result);

        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let expected_output = vec![
            vec![14, 8, 2],
            vec![13, 7, 1],
            vec![12, 6],
            vec![11, 5],
            vec![10, 4],
            vec![9, 3],
        ];
        let result = get_filings_for_threads(&mut input, 6);
        assert_eq!(expected_output, result);
    }

}
