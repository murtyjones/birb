// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*               # sometimes there's whitespace before
        net
        \s+
        income
        \s+
        attributable
        \s+
        to
        \s+
        (non[-–—\s]+)*controlling
        \s+
        interests
        \s*               # sometimes there's whitespace after
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
            "Net income attributable to controlling interests",
            "Net income attributable to non-controlling interests",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["legal fees"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
