use futures::{Future, Stream};
use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::fmt;
use std::time::{Duration, Instant};

#[derive(Copy, Clone)]
pub enum Year {
    TwentySixteen = 2016,
    TwentySeventeen = 2017,
    TwentyEighteen = 2018,
    TwentyNineteen = 2019,
}

#[derive(Copy, Clone)]
pub enum Quarter {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            _ => *self as i32,
        };
        write!(f, "{}", printable)
    }
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            _ => *self as i32,
        };
        write!(f, "{}", printable)
    }
}

/// Gets an index file given a quarter and year
pub fn main(q: Quarter, y: Year) -> Vec<u8> {
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(200));
    let client = S3Client::new_with(
        HttpClient::new().expect("failed to create request dispatcher"),
        chain,
        Region::UsEast1,
    );
    let bucket = format!("birb-edgar-indexes");
    let filename = format!("{}/QTR{}/master.idx", y, q);
    get_object(&client, &bucket, &filename)
}

#[cfg(not(test))]
/// Gets an S3 object
fn get_object(client: &S3Client, bucket: &str, filename: &str) -> Vec<u8> {
    let get_req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: filename.to_owned(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req)
        .sync()
        .expect("Couldn't GET object");

    let stream = result.body.unwrap();
    let body = stream.concat2().wait().unwrap();

    assert!(body.len() > 0);
    body
}

#[cfg(test)]
fn get_object(_c: &S3Client, _b: &str, _f: &str) -> Vec<u8> {
    // TODO make this return a real index document
    // for parser testing
    vec![1, 2, 3]
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
