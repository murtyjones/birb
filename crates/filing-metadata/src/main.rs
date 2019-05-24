#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate futures;

pub mod download_index;
mod parse_index;
mod persist_filing_metadatas;
mod should_process_for_quarter;
pub mod time_periods;

use should_process_for_quarter::ShouldProcess;
use time_periods::Quarter;
use time_periods::Year;

fn main() {
    // 2016
    do_for_time_period(Quarter::One, Year::TwentySixteen);
    do_for_time_period(Quarter::Two, Year::TwentySixteen);
    do_for_time_period(Quarter::Three, Year::TwentySixteen);
    do_for_time_period(Quarter::Four, Year::TwentySixteen);

    // 2017
    do_for_time_period(Quarter::One, Year::TwentySeventeen);
    do_for_time_period(Quarter::Two, Year::TwentySeventeen);
    do_for_time_period(Quarter::Three, Year::TwentySeventeen);
    do_for_time_period(Quarter::Four, Year::TwentySeventeen);

    // 2018
    do_for_time_period(Quarter::One, Year::TwentyEighteen);
    do_for_time_period(Quarter::Two, Year::TwentyEighteen);
    do_for_time_period(Quarter::Three, Year::TwentyEighteen);
    do_for_time_period(Quarter::Four, Year::TwentyEighteen);

    // TODO 2019
}

fn do_for_time_period(q: Quarter, y: Year) {
    let should_process = should_process_for_quarter::main(q, y);
    match should_process {
        Ok(ShouldProcess::Yes) => {
            let index_contents = download_index::main(q, y);
            // Some indexes have invalid utf8 characters - e.g. 3Q17,
            // so we convert "lossy" style:
            let str_index_contents = String::from_utf8_lossy(&index_contents).into_owned();
            let filing_metadatas =
                parse_index::main(str_index_contents).expect("Unable to parse index file");
            persist_filing_metadatas::main(q, y, filing_metadatas);
            println!("{}Q{} finished processing.", q, y);
        }
        Ok(ShouldProcess::No) => println!("{}Q{} already processed.", q, y),
        Err(e) => {
            panic!("Couldn't process for {}Q{}", q, y);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
