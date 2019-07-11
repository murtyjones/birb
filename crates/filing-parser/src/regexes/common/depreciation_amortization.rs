// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                           # sometimes there's whitespace before
        depreciation
        (
            ,\s+amortization,\s+and\s+decommissioning
            |
            \s+and\s+amortization
        )*
        (\s+expense)*
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
            "Depreciation, amortization, and decommissioning",
            "depreciation",
            "  depreciation",
            "Depreciation and amortization expense",
            "Depreciation expense",
            "Depreciation and amortization",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["blah depreciation blah"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}