use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use postgres::params::IntoConnectParams;
use postgres::{Connection, TlsMode};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode as R2d2TlsMode};
use std::fs::File;
use std::fs::{self, ReadDir};
use std::io::prelude::*;
use std::io::Error;

pub fn get_cik(filing_url: &str) -> String {
    let split = filing_url.split("/");
    let split = split.collect::<Vec<&str>>();
    assert_eq!(
        4,
        split.len(),
        "provided filing url is not correctly formatted! ({})",
        filing_url
    );
    String::from(split[2])
}

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

pub fn get_connection_pool<S>(host: S) -> Pool<PostgresConnectionManager>
where
    S: IntoConnectParams,
{
    let manager = PostgresConnectionManager::new(host, R2d2TlsMode::None).unwrap();
    Pool::new(manager).unwrap()
}

pub fn decompress_gzip(compressed: Vec<u8>) -> String {
    let mut d = GzDecoder::new(compressed.as_slice());
    let mut decompressed_object_contents = String::new();
    d.read_to_string(&mut decompressed_object_contents).unwrap();
    decompressed_object_contents
}

pub fn compress_gzip(compressed: Vec<u8>) -> Vec<u8> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(compressed.as_slice())
        .expect("Couldn't write to gzip");
    e.finish().expect("Couldn't complete gzip writing")
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

pub fn get_content_type_from_filepath(file_path: &str) -> Option<&str> {
    let split = file_path.split(".");
    let split = split.collect::<Vec<&str>>();
    let last_chunk = split[split.len() - 1];

    match last_chunk {
        "htm" | "html" => Some("text/html; charset=utf-8"),
        "css" => Some("text/css; charset=utf-8"),
        "xml" => Some("text/xml; charset=utf-8"),
        "js" => Some("text/javascript; charset=UTF-8"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "gif" => Some("image/gif"),
        "png" => Some("image/png"),
        "tiff" => Some("image/tiff"),
        "bmp" => Some("image/bmp"),
        "ico" => Some("image/x-icon"),
        "svg" => Some("image/svg+xml"),
        "zip" => Some("application/zip, application/octet-stream"),
        "xlsx" => {
            Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet; charset=utf-8")
        }
        _ => None,
    }
}

mod test {
    use super::*;

    #[test]
    fn test_get_content_type_from_filepath() {
        let file_path = "edgar/data/1322952/0001017386-17-000104/R42.htm";
        assert_eq!(
            "text/html; charset=utf-8",
            get_content_type_from_filepath(file_path).unwrap()
        );
        let file_path = "edgar/data/1322952/0001017386-17-000104/report.css";
        assert_eq!(
            "text/css; charset=utf-8",
            get_content_type_from_filepath(file_path).unwrap()
        );
        let file_path = "edgar/data/1322952/0001017386-17-000104/Financial_Report.xlsx";
        assert_eq!(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet; charset=utf-8",
            get_content_type_from_filepath(file_path).unwrap()
        );
        let file_path = "edgar/data/1322952/0001017386-17-000104/FilingSummary.xml";
        assert_eq!(
            "text/xml; charset=utf-8",
            get_content_type_from_filepath(file_path).unwrap()
        );
        let file_path = "edgar/data/1322952/0001017386-17-000104/anton_chia-logo.jpg";
        assert_eq!(
            "image/jpeg",
            get_content_type_from_filepath(file_path).unwrap()
        );
        let file_path = "edgar/data/1322952/0001017386-17-000104/anton_chia-logo.jpeg";
        assert_eq!(
            "image/jpeg",
            get_content_type_from_filepath(file_path).unwrap()
        );
    }

    #[test]
    fn test_get_accession_number() {
        let url = "edgar/data/1143513/0001193125-16-453914.txt"; // random but real example
        let r = get_accession_number(url);
        assert_eq!("0001193125-16-453914", r);
    }

    #[test]
    fn test_get_cik() {
        let url = "edgar/data/1143513/0001193125-16-453914.txt"; // random but real example
        let r = get_cik(url);
        assert_eq!("1143513", r);
    }
}
