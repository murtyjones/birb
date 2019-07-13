#[macro_use]
extern crate lazy_static;
extern crate regex;
use aws::s3;
use filing_parser::master_processor::ParsedFiling;
use rand::Rng;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use rusoto_s3::{Object, S3Client};

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
    let data = s3::list_s3_objects(&client, BUCKET);
    //    panic!("{}", data.keys);
    let starting = 0; // <-- change this to whatever the last failing doc was.

    data[starting..]
        .par_iter()
        .enumerate()
        .for_each(|(i, object)| {
            println!("Starting for index: {} / key: {:?}", i, object.key);
            process(&client, object);
        });
}

fn process(client: &S3Client, data: &Object) {
    let object_key = data.key.as_ref().expect("Object should have a key!");
    //    let random_object_key = &String::from("edgar/data/1011509/0001104659-16-100057.txt");

    let object = s3::get_s3_object(client, BUCKET, object_key);

    println!("Object retrieved...");

    let contents = String::from_utf8(object).unwrap();
    //    if TEN_K_REGEX.is_match(&*contents) {
    //        println!("10-K, skipping");
    //        return ();
    //    }

    println!("Processing...");

    let processed = ParsedFiling::new(contents, object_key.to_string()).unwrap();

    // TODO: This part is crazy slow! How can it be sped up when it comes time to write processed files to S3?
    //    return write_to_file(processed);

    println!("Done");
}

fn write_to_file(mut parsed: ParsedFiling) {
    let path = String::from("examples/10-Q/output/wow.html");
    println!("Processed! Writing to file...");
    parsed.write_file_contents(&path);
}
