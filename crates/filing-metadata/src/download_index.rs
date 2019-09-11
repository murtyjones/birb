use futures::{Future, Stream};
use rusoto_core::credential::{ChainProvider, InstanceMetadataProvider};
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use std::time::{Duration, Instant};

use crate::aws::s3::get_s3_client;
use crate::time_periods::Quarter;
use crate::time_periods::Year;

/// Gets an index file given a quarter and year
pub fn main(q: Quarter, y: Year) -> Vec<u8> {
    let bucket = format!("birb-edgar-indexes");
    let filename = format!("{}/QTR{}/master.idx", y, q);
    let client = get_s3_client();
    get_s3_object(&client, &bucket, &filename)
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
    body.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        main(Quarter::Two, Year::TwentySeventeen);
    }

    #[test]
    fn test_quarters() {
        assert_eq!(1, Quarter::One as i32);
        assert_eq!(2, Quarter::Two as i32);
        assert_eq!(3, Quarter::Three as i32);
        assert_eq!(4, Quarter::Four as i32);
    }

    #[test]
    fn test_years() {
        assert_eq!(2016, Year::TwentySixteen as i32);
        assert_eq!(2017, Year::TwentySeventeen as i32);
        assert_eq!(2018, Year::TwentyEighteen as i32);
        assert_eq!(2019, Year::TwentyNineteen as i32);
    }
}
