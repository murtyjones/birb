#[macro_use]
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
#[macro_use]
extern crate log;
extern crate aws;
extern crate env_logger;

extern crate futures;

pub mod download_index;
mod parse_index;
mod persist_filing_metadatas;
pub mod should_process_for_quarter;
pub mod time_periods;

use should_process_for_quarter::ShouldProcess;
use time_periods::Quarter;
use time_periods::Year;

pub fn do_for_time_period(q: Quarter, y: Year) {
    let should_process = should_process_for_quarter::main(q, y);
    match should_process {
        Ok(ShouldProcess::Yes) => {
            info!("About to process {}Q{}", q, y);
            let index_contents = download_index::main(q, y);
            info!("got contents");
            // Some indexes have invalid utf8 characters - e.g. 3Q17,
            // so we convert "lossy" style:
            let str_index_contents = String::from_utf8_lossy(&index_contents).into_owned();
            info!("from lossy retrieved");
            let filing_metadatas =
                parse_index::main(str_index_contents).expect("Unable to parse index file");
            info!("data parsed");
            persist_filing_metadatas::main(q, y, filing_metadatas);
            info!("{}Q{} finished processing", q, y);
        }
        Ok(ShouldProcess::No) => info!("{}Q{} already processed", q, y),
        Err(e) => {
            panic!("Couldn't process for {}Q{}", q, y);
        }
    }
}

pub fn do_for_all_time_periods() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {}
}
