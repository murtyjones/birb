// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                           # sometimes there's whitespace before
        (
            (
                (
                    (net\s+)*(operating\s+)*income
                    |
                    loss
                    |
                    income(\s+|/)\(loss\)
                )\s+
                (\(loss\)\s+)*from\s+(continuing\s+)*operations
            )
            |
            operating\s+income
        )
        \s*                                           # sometimes there's whitespace after
        $
    ";
    pub static ref REGEX: Regex = RegexBuilder::new(&PATTERN)
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
    fn test() {
        let match_examples = vec![
            "Income From Continuing Operations",
            "Net income from continuing operations",
            "Net income (loss) from continuing operations",
            "Loss from continuing operations",
            "Net operating income (loss) from continuing operations",
            "Loss from operations",
            "  Operating income",
            "Income/(Loss) from operations",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["net income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
