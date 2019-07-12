// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                           # sometimes there's whitespace before
        (
            total\s+
            |
            income\s+and\s+
        )*
        revenue(s)*
        (\s+and\s+other\s+income)*
        (:)*
        \s*                                           # sometimes there's whitespace af
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
            "revenue",
            "   revenues",
            "total revenue",
            "total revenues",
            "revenue:",
            "Income and Revenues:",
            "Revenues and other income:",
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
