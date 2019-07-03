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
}

lazy_static! {
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

lazy_static! {
    static ref INTEREST_INCOME_PATTERN: &'static str = r"
        interest\s+income
    ";
    pub static ref INTEREST_INCOME_REGEX: Regex = RegexBuilder::new(&INTEREST_INCOME_PATTERN)
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
        let match_examples = vec!["For the nine months ended", "Months ended"];
        for each in match_examples {
            assert!(MONTHS_ENDED_REGEX.is_match(each));
        }
        let no_match_examples = vec!["For the month of May", "Moooonths ended"];
        for each in no_match_examples {
            assert!(!MONTHS_ENDED_REGEX.is_match(each));
        }
    }

    #[test]
    fn test_shares_oustanding() {
        let match_examples = vec![
            "Weighted average number of shares outstanding",
            "Weighted average shares outstanding - basic",
            "Weighted average shares outstanding - diluted",
        ];
        for each in match_examples {
            assert!(SHARES_OUTSTANDING_REGEX.is_match(each));
        }
        let no_match_examples = vec!["average shares"];
        for each in no_match_examples {
            assert!(!SHARES_OUTSTANDING_REGEX.is_match(each));
        }
    }

    #[test]
    fn test_interest_income() {
        let match_examples = vec!["Interest income", "interest income"];
        for each in match_examples {
            assert!(INTEREST_INCOME_REGEX.is_match(each));
        }
        let no_match_examples = vec!["interest expense"];
        for each in no_match_examples {
            assert!(!INTEREST_INCOME_REGEX.is_match(each));
        }
    }
}
