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

mod test {
    use super::*;

    #[test]
    fn test_get_accession_number() {
        let url = "edgar/data/1143513/0001193125-16-453914.txt"; // random but real example
        let r = get_accession_number(url);
        assert_eq!("0001193125-16-453914", r);
    }
}
