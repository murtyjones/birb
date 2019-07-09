// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (
            net\s+income(\s+\(loss\))*
            |
            net\s+loss
        )
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
    fn test_earnings_per_share() {
        let match_examples = vec!["Net income", "Net income (loss)", "Net loss"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share", "income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
