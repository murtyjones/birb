use aws::s3;
use filing_parser::helpers::write_to_file;
use filing_parser::ten_q::ProcessedFiling;
use rand::Rng;

const BUCKET: &'static str = "birb-edgar-filings";

/// Intended to randomly process files from S3
fn main() {
    let client = s3::get_s3_client();
    let data = s3::list_s3_objects(&client, BUCKET);
    let mut i = rand::thread_rng().gen_range(0, data.len() - 1);
    let random_object_key = data[i].key.as_ref().expect("Object should have a key!");
    println!("Random object to process: {:?}", &random_object_key);
    let object = s3::get_s3_object(&client, BUCKET, random_object_key);
    let mut processed = ProcessedFiling::new(String::from_utf8(object).unwrap()).unwrap();
    let path = String::from("examples/10-Q/input/wow.html");
    processed.write_file_contents(&path);
    println!("Done");
}
