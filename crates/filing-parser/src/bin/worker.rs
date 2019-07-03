use aws::s3;
use filing_parser::ten_q::ProcessedFiling;
use rand::Rng;

const BUCKET: &'static str = "birb-edgar-filings";

/// Intended to randomly process files from S3
fn main() {
    let client = s3::get_s3_client();
    let data = s3::list_s3_objects(&client, BUCKET);
    let i = rand::thread_rng().gen_range(0, data.len() - 1);
    let random_object_key = data[i].key.as_ref().expect("Object should have a key!");
    let random_object_key = &String::from("edgar/data/1004980/0001004980-16-000073.txt");

    println!("Random object to process: {:?}", &random_object_key);

    let object = s3::get_s3_object(&client, BUCKET, random_object_key);

    println!("Object retrieved...");

    let contents = String::from_utf8(object).unwrap();
    if contents.contains("    10-K") {
        println!("10-K, skipping");
        return ();
    }

    println!("Processing...");

    let mut processed = ProcessedFiling::new(contents).unwrap();
    let path = String::from("examples/10-Q/output/wow.html");

    println!("Processed! Writing to file...");

    // TODO: This part is crazy slow! How can it be sped up when it comes time to write processed files to S3?
    processed.write_file_contents(&path);

    println!("Done");
}
