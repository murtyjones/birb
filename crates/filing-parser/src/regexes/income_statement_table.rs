// regex
use regex::{Regex, RegexBuilder};

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
    static ref SHARES_OUTSTANDING_PATTERN: &'static str = r"
        Weighted\s+
        average\s+
        (number\s+of\s+)*
        shares\s+
        outstanding
        .*
    ";
    pub static ref SHARES_OUTSTANDING_REGEX: Regex = RegexBuilder::new(&SHARES_OUTSTANDING_PATTERN)
        .case_insensitive(true)
        .multi_line(true)
        .ignore_whitespace(true)
        .build()
        .expect("Couldn't build income statement regex!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_months_ended() {
        let examples = vec!["For the nine months ended", "Months ended"];
        for each in examples {
            assert!(MONTHS_ENDED_REGEX.is_match(each));
        }
    }

    #[test]
    fn test_shares_oustanding() {
        let examples = vec![
            "Weighted average number of shares outstanding",
            "Weighted average shares outstanding - basic",
            "Weighted average shares outstanding - diluted",
        ];
        for each in examples {
            assert!(SHARES_OUTSTANDING_REGEX.is_match(each));
        }
    }
}
