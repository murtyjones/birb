use futures::{Future, Stream};
use rusoto_core::credential::{ChainProvider, InstanceMetadataProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, ListObjectsRequest, Object, PutObjectRequest, S3Client, S3};

pub fn get_s3_client() -> S3Client {
    let credentials = ChainProvider::new();
    S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        credentials,
        Region::UsEast1,
    )
}

/// Gets an S3 object
pub fn get_s3_object(client: &S3Client, bucket: &str, filename: &str) -> Vec<u8> {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET S3 object");

    let stream = result.body.unwrap();
    let body = stream.concat2().wait().unwrap();

    assert!(body.len() > 0);
    body
}

pub fn list_s3_objects(client: &S3Client, bucket: &str) -> Vec<Object> {
    let list_req = ListObjectsRequest {
        bucket: bucket.to_owned(),
        delimiter: None,
        encoding_type: None,
        marker: None,
        max_keys: None,
        prefix: None,
        request_payer: None,
    };

    let result = client
        .list_objects(list_req)
        .sync()
        .expect("Couldn't list S3 objects");

    result.contents.expect("Bucket should not be empty!")
}
