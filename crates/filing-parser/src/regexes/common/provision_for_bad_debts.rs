// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*            # sometimes there's whitespace before
        Provision
        \s+
        for
        (
            \s+bad\s+debts
            |
            \s+credit\s+losses
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
    fn test() {
        let match_examples = vec!["provision for bad debts", "Provision for credit losses"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["long term debt", "debt"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
