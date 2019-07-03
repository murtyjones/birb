// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref SHARES_OUTSTANDING_PATTERN: &'static str = r"
        (weighted\s+)*
        average\s+
        (number\s+of\s+)*
        (common\s+)*
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
    static ref SHARES_USED_PATTERN: &'static str = r"
        Shares\s+
        used\s+
        in\s+
        per\s+
        share\s+
        calculation
        (:)*
    ";
    pub static ref SHARES_USED_REGEX: Regex = RegexBuilder::new(&SHARES_USED_PATTERN)
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

lazy_static! {
    static ref EARNINGS_PER_SHARE_PATTERN: &'static str = r"
        (basic and diluted\s+)*
        (earnings|loss)\s+
        (\(loss\)\s+)*
        per\s+
        share\s*
        (:)*
    ";
    pub static ref EARNINGS_PER_SHARE_REGEX: Regex = RegexBuilder::new(&EARNINGS_PER_SHARE_PATTERN)
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
    fn test_shares_oustanding() {
        let match_examples = vec![
            "Weighted average number of shares outstanding",
            "Weighted average shares outstanding - basic",
            "Weighted average shares outstanding - diluted",
            "Weighted average common shares outstanding:",
            "Average shares outstanding (basic)",
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
    fn test_shares_used() {
        let match_examples = vec!["Shares used in per share calculation:"];
        for each in match_examples {
            assert!(SHARES_USED_REGEX.is_match(each));
        }
        let no_match_examples = vec!["average shares"];
        for each in no_match_examples {
            assert!(!SHARES_USED_REGEX.is_match(each));
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

    #[test]
    fn test_earnings_per_share() {
        let match_examples = vec![
            "Earnings (loss) per share:",
            "Earnings per share (basic)",
            "Basic and diluted loss per share",
        ];
        for each in match_examples {
            assert!(EARNINGS_PER_SHARE_REGEX.is_match(each));
        }
        let no_match_examples = vec!["net earnings"];
        for each in no_match_examples {
            assert!(!EARNINGS_PER_SHARE_REGEX.is_match(each));
        }
    }
}
