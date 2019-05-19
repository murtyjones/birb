extern crate futures;

mod download_index;
mod parse_index;
mod persist_filing_metadatas;
mod time_periods;

use time_periods::Quarter;
use time_periods::Year;

fn main() {
    do_for_time_period(Quarter::One, Year::TwentySixteen);
}

fn do_for_time_period(q: Quarter, y: Year) {
    let index_contents = download_index::main(q, y);
    let str_index_contents = String::from_utf8(index_contents).expect("Found invalid UTF-8");
    let filing_metadatas =
        parse_index::main(str_index_contents).expect("Unable to parse index file");
    persist_filing_metadatas::main(q, y, filing_metadatas);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
