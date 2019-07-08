// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref EARNINGS_PER_SHARE_PATTERN: &'static str = r"
       ^
        (basic\s+and\s+diluted\s+)*
        (
            net\s+income
            |
            earnings(\s+\(loss\))*
            |
            loss
        )
        \s+
        per
        \s+
        share
        (
             (\s+\(basic\))
             |
             (\s+[-–—]\s+(diluted|basic))
        )*
        (:)*
        $
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
    fn test_earnings_per_share() {
        let match_examples = vec![
            "Earnings (loss) per share:",
            "Earnings per share (basic)",
            "Basic and diluted loss per share",
            "Net income per share – diluted",
            "Net income per share - basic",
        ];
        for each in match_examples {
            assert!(EARNINGS_PER_SHARE_REGEX.is_match(each));
        }
        let no_match_examples = vec!["net earnings", "Note 12—Earnings Per Share"];
        for each in no_match_examples {
            assert!(!EARNINGS_PER_SHARE_REGEX.is_match(each));
        }
    }
}
