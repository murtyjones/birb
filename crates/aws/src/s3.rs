use failure;
use futures::{Future, Stream};
use rusoto_core::credential::{ChainProvider, InstanceMetadataProvider, ProfileProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::ProvideAwsCredentials;
use rusoto_s3::util::PreSignedRequest;
use rusoto_s3::{GetObjectRequest, ListObjectsRequest, Object, PutObjectRequest, S3Client, S3};
use utils::{compress_gzip, get_content_type_from_filepath};

pub fn get_birb_region() -> Region {
    Region::UsEast1
}

pub fn get_birb_credentials() -> ChainProvider {
    let mut p = ProfileProvider::new()
        .ok()
        .expect("Couldn't make ProfilePrivder");
    p.set_profile("birb");
    #[cfg(debug_assertions)]
    let credentials = ChainProvider::with_profile_provider(p);
    #[cfg(not(debug_assertions))]
    let credentials = ChainProvider::new();
    // if the above doesn't work, try: let mut credentials = InstanceMetadataProvider::new();
    credentials
}

pub fn get_s3_client() -> S3Client {
    S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        get_birb_credentials(),
        get_birb_region(),
    )
}

/// Gets an S3 object
pub fn get_s3_object<S: Into<String>>(client: &S3Client, bucket: S, filename: S) -> Vec<u8> {
    let get_req = GetObjectRequest {
        bucket: bucket.into().to_owned(),
        key: filename.into().to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET S3 object");

    let stream = result.body.unwrap();
    let body = stream.concat2().wait().unwrap();

    assert!(body.len() > 0);
    body.to_vec()
}

pub fn list_s3_objects<S: Into<String>>(client: &S3Client, bucket: S) -> Vec<Object> {
    let list_req = ListObjectsRequest {
        bucket: bucket.into().to_owned(),
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

pub fn get_signed_url<S: Into<String>>(bucket: S, filename: S) -> String {
    let req = GetObjectRequest {
        bucket: bucket.into().to_owned(),
        key: filename.into().to_owned(),
        ..Default::default()
    };

    let credentials = &futures::Future::wait(get_birb_credentials().credentials())
        .expect("Couldn't get credentials!");

    req.get_presigned_url(&get_birb_region(), credentials, &Default::default())
}

pub fn store_s3_document_gzipped(
    client: &S3Client,
    bucket: &str,
    file_path: &str,
    contents: Vec<u8>,
) -> Result<(), failure::Error> {
    let compressed_contents = compress_gzip(contents);

    let content_type = get_content_type_from_filepath(&file_path);
    let content_type = content_type.map(|s| s.to_string());

    let put_req = PutObjectRequest {
        bucket: bucket.to_owned(),
        key: file_path.to_owned(),
        body: Some(compressed_contents.into()),
        content_encoding: Some(String::from("gzip")),
        content_type,
        ..Default::default()
    };
    client
        .put_object(put_req)
        .sync()
        .expect("Couldn't PUT S3 object.");
    Ok(())
}
