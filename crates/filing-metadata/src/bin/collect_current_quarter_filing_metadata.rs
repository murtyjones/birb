extern crate chrono;
extern crate filing_metadata;
use crate::chrono::Datelike;

use filing_metadata::time_periods::Quarter;
use filing_metadata::time_periods::Year;

fn main() {
    let current_qtr = get_current_quarter();
    let current_year = get_current_year();
    // download index from sec.gov
    // parse it
    // sql transaction: insert into filings ignore conflicts
    // NOTE: don't forget logging

    // once built:
    // run for Q1, Q2 and Q3 2019 to ensure they are up to date
    // build a lambda that runs this every hour
}

fn get_current_year() -> i32 {
    let now = chrono::Utc::now();
    now.year()
}

fn get_current_quarter() -> i32 {
    let month = chrono::Utc::now().month();
    match month {
        1|2|3 => 1,
        4|5|6 => 2,
        7|8|9 => 3,
        _ /* IE. 10|11|12*/ => 4,
    }
}
