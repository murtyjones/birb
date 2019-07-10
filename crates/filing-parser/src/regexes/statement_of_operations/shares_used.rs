// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (weighted\s+average\s+number\s+of\s+common\s+)*
        shares
        \s+
        used
        \s+
        in
        \s+
        (
            per\s+share\s+(calculation|computations)
            |
            computation\s+of\s+per\s+common\s+share\s+data
        )
        (:)*
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
            "Shares used in per share calculation:",
            "Shares used in computation of per common share data:",
            "Weighted average number of common shares used in per share computations",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["average shares"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
