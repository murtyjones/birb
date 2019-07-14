#[macro_use]
extern crate lazy_static;
extern crate regex;
use aws::s3;
use filing_parser::master_processor::ParsedFiling;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use rusoto_s3::{ListObjectsRequest, Object, S3Client, S3};

const BUCKET: &'static str = "birb-edgar-filings";

lazy_static! {
    static ref TEN_K_PATTERN: &'static str = r"CONFORMED SUBMISSION TYPE:\s+10-K";
    pub static ref TEN_K_REGEX: Regex = RegexBuilder::new(&TEN_K_PATTERN)
        .case_insensitive(true)
        .build()
        .expect("Couldn't build income statement regex!");
}

/// Intended to randomly process files from S3
fn main() {
    let client = s3::get_s3_client();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let object_key = &args[1];
        let object = s3::get_s3_object(&client, BUCKET, object_key);
        let contents = String::from_utf8(object).unwrap();
        ParsedFiling::new(contents, object_key.to_string()).unwrap();
        return ();
    }

    let starting_key = "edgar/data/1029800/0001029800-16-000063.txt"; // <- item 999
    let starting_key = "edgar/data/1060391/0001060391-18-000005.txt"; // <- item 1,998
    let starting_key = "edgar/data/1094739/0001094739-16-000209.txt"; // <- item 2,997
    let data = get_next_1000_filing_keys(&client, BUCKET, starting_key);

    data.par_iter().enumerate().for_each(|(i, object)| {
        println!("Starting for index: {} / key: {:?}", i, object.key);
        process(&client, object);
    });
}

fn process(client: &S3Client, data: &Object) {
    let object_key = data.key.as_ref().expect("Object should have a key!");
    //    let random_object_key = &String::from("edgar/data/1011509/0001104659-16-100057.txt");

    let object = s3::get_s3_object(client, BUCKET, object_key);

    let contents = String::from_utf8(object).unwrap();
    //    if TEN_K_REGEX.is_match(&*contents) {
    //        println!("10-K, skipping");
    //        return ();
    //    }

    let processed = ParsedFiling::new(contents, object_key.to_string()).unwrap();

    // TODO: This part is crazy slow! How can it be sped up when it comes time to write processed files to S3?
    //    return write_to_file(processed);
}

fn write_to_file(mut parsed: ParsedFiling) {
    let path = String::from("examples/10-Q/output/wow.html");
    println!("Processed! Writing to file...");
    parsed.write_file_contents(&path);
}

fn get_next_1000_filing_keys(
    client: &S3Client,
    bucket: &'static str,
    starting_key: &'static str,
) -> Vec<Object> {
    let list_req = ListObjectsRequest {
        bucket: bucket.to_owned(),
        delimiter: None,
        encoding_type: None,
        marker: Some(starting_key.to_owned()),
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

// edgar/data/1045929/0001354488-16-007734.txt
// edgar/data/1045929/0001552781-17-000010.txt
// edgar/data/1045929/0001552781-17-000012.txt
// edgar/data/1045942/0001415889-16-004686.txt
// edgar/data/1038509/0001038509-17-000016.txt
// edgar/data/1040721/0001493152-16-009422.txt
// edgar/data/1043565/0001193125-16-519598.txt
// edgar/data/1043565/0001193125-16-588829.txt
// edgar/data/1043565/0001193125-17-166787.txt
// edgar/data/1043565/0001193125-17-254024.txt
// edgar/data/1043565/0001193125-18-098996.txt
// edgar/data/1059272/0001059272-16-000113.txt
// edgar/data/1059272/0001059272-16-000143.txt
// edgar/data/1037189/0001193125-16-519569.txt
// edgar/data/1059272/0001059272-17-000044.txt
// edgar/data/1059376/0001193125-17-101629.txt
// edgar/data/1059377/0001193125-18-101710.txt
// edgar/data/1059378/0001193125-17-101669.txt
// edgar/data/1059378/0001193125-18-101649.txt
// edgar/data/1036848/0001036848-16-000082.txt
// edgar/data/1057051/0001193125-16-519595.txt
// edgar/data/1057051/0001193125-17-099664.txt
// edgar/data/1057051/0001193125-17-166796.txt
// edgar/data/1057051/0001193125-18-098930.txt
