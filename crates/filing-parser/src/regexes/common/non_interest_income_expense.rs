// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*               # sometimes there's whitespace before
        (total\s+)*
        (non-interest|noninterest)
        \s+
        (
            income(\s+\(loss\))*
            |
            expense
        )
        (:)*
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
            "Non-interest income (loss):",
            "Non-interest expense",
            "Non-interest expense:",
            "Noninterest Income",
            "Non-Interest Income:",
            "Total non-interest income	",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["Net—Interest Income", "Interest income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
