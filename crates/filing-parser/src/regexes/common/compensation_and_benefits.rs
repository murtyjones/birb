// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                              # Sometimes there's whitespace before
        compensation
        \s+
        and
        \s+
        benefits
        (:)*
        \s*                              # Sometimes there's whitespace after
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
        let match_examples = vec!["Compensation and benefits"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
