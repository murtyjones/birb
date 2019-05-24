extern crate filing_metadata;
use filing_metadata::do_for_time_period;
use filing_metadata::time_periods::Quarter;
use filing_metadata::time_periods::Year;

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
