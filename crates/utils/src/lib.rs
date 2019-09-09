use flate2::read::GzDecoder;
use postgres::{Connection, TlsMode};
use std::fs::File;
use std::fs::{self, ReadDir};
use std::io::prelude::*;
use std::io::Error;

pub fn get_accession_number(filing_url: &str) -> String {
    let split = filing_url.split("/");
    let split = split.collect::<Vec<&str>>();
    assert_eq!(
        4,
        split.len(),
        "provided filing url is not correctly formatted! ({})",
        filing_url
    );
    let split = split[3].split(".");
    let split = split.collect::<Vec<&str>>();
    assert_eq!(
        2,
        split.len(),
        "provided filing url is not correctly formatted! ({})",
        filing_url
    );
    String::from(split[0])
}

pub fn write_to_file(
    file_path: &String,
    extension: &'static str,
    data: Vec<u8>,
) -> std::io::Result<()> {
    let path = std::path::PathBuf::from(format!("{}{}", file_path, extension));
    let mut pos = 0;
    println!("Path: {:?}", path);
    let mut buffer = File::create(path).expect("Couldn't make file");

    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

pub fn get_connection<S>(host: S) -> Connection
where
    S: Into<String>,
{
    Connection::connect(host.into(), TlsMode::None).unwrap()
}

mod test {
    use super::*;

    #[test]
    fn test_get_accession_number() {
        let url = "edgar/data/1143513/0001193125-16-453914.txt"; // random but real example
        let r = get_accession_number(url);
        assert_eq!("0001193125-16-453914", r);
    }
}

pub fn decompress_gzip(compressed: Vec<u8>) -> String {
    let mut d = GzDecoder::new(compressed.as_slice());
    let mut decompressed_object_contents = String::new();
    d.read_to_string(&mut decompressed_object_contents).unwrap();
    decompressed_object_contents
}

pub fn delete_dir_contents(path: &str) {
    let read_dir_res = fs::read_dir(path);
    if let Ok(dir) = read_dir_res {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
}
