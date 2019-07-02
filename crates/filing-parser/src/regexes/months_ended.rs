// regex
use regex::{Regex, RegexBuilder};
use std::ascii::escape_default;

// test files
use crate::test_files::get_files;

lazy_static! {
    static ref MONTHS_ENDED_PATTERN: &'static str = r"
        .*
        months\s+
        ended\s*
        .*
    ";
    pub static ref MONTHS_ENDED_REGEX: Regex = RegexBuilder::new(&MONTHS_ENDED_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .build()
        .expect("Couldn't build income statement regex!");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_files::MatchType;

    /// test examples for which we dont want to process the full doc
    #[test]
    fn test_further_examples() {
        let examples = vec!["For the nine months ended", "Months ended"];
        for each in examples {
            assert!(MONTHS_ENDED_REGEX.is_match(each));
        }
    }
}
