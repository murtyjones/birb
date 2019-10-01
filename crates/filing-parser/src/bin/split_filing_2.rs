use aws::s3::get_s3_client;
use futures::{future, Future, Stream};
use rusoto_s3::S3;
use rusoto_s3::{GetObjectOutput, GetObjectRequest};
use tokio_core::reactor::Core;

const FILENAMES: &'static [&'static str] = &[
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
    "edgar/data/1001463/0001185185-17-000668.txt.gz",
];

fn main() {
    requests_async();
    requests_sync();
}

fn requests_sync() {
    let now = std::time::Instant::now();

    let s3_client = get_s3_client();

    let results: Vec<GetObjectOutput> = FILENAMES
        .iter()
        .map(|f| {
            let get_req = GetObjectRequest {
                bucket: String::from("birb-edgar-filings"),
                key: String::from(*f),
                ..Default::default()
            };
            s3_client
                .get_object(get_req)
                .sync()
                .expect("Couldn't GET S3 object")
        })
        .collect::<Vec<GetObjectOutput>>();

    println!("Finished in {:?}", now.elapsed());
}

fn requests_async() {
    let now = std::time::Instant::now();

    let mut core = Core::new().unwrap();

    let s3_client = get_s3_client();

    let promises = future::join_all(FILENAMES.iter().map(|f| {
        let get_req = GetObjectRequest {
            bucket: String::from("birb-edgar-filings"),
            key: String::from(*f),
            ..Default::default()
        };
        s3_client.get_object(get_req)
    }));

    let results: Vec<GetObjectOutput> = match core.run(promises) {
        Ok(items) => items,
        Err(e) => panic!("Error completing futures: {}", e),
    };

    println!("Finished in {:?}", now.elapsed());
}
