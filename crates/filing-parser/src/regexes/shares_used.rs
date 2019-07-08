// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref SHARES_USED_PATTERN: &'static str = r"
        shares
        \s+
        used
        \s+
        in
        \s+
        (
            per\s+share\s+calculation
            |
            computation\s+of\s+per\s+common\s+share\s+data
        )
        (:)*
    ";
    pub static ref SHARES_USED_REGEX: Regex = RegexBuilder::new(&SHARES_USED_PATTERN)
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
    fn test_shares_used() {
        let match_examples = vec![
            "Shares used in per share calculation:",
            "Shares used in computation of per common share data:",
        ];
        for each in match_examples {
            assert!(SHARES_USED_REGEX.is_match(each));
        }
        let no_match_examples = vec!["average shares"];
        for each in no_match_examples {
            assert!(!SHARES_USED_REGEX.is_match(each));
        }
    }
}
