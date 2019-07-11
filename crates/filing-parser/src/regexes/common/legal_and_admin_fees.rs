// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        legal
        \s+
        and
        \s+
        (administrative|professional)
        \s+
        fees
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
            "Legal and administrative fees",
            "Legal and professional fees",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["non-legal fees"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
