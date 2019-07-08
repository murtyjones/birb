// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref SHARES_OUTSTANDING_PATTERN: &'static str = r"
        (weighted\s+)*
        average\s+
        (number\s+of\s+)*
        (common\s+)*
        shares\s+
        (outstanding\s*)*
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
    fn test_shares_outstanding() {
        let match_examples = vec![
            "Weighted average number of shares outstanding",
            "Weighted average shares outstanding - basic",
            "Weighted average shares – basic and diluted",
            "Weighted average shares outstanding - diluted",
            "Weighted average common shares outstanding:",
            "Average shares outstanding (basic)",
            "Weighted average number of common shares outstanding - basic and diluted",
            "Weighted-average common shares outstanding",
            "Weighted average shares outstanding:",
        ];
        for each in match_examples {
            assert!(SHARES_OUTSTANDING_REGEX.is_match(each));
        }
        let no_match_examples = vec![
            "average shares",
            "earnings per share",
            "shares held",
            "shares",
        ];
        for each in no_match_examples {
            assert!(!SHARES_OUTSTANDING_REGEX.is_match(each));
        }
    }
}
