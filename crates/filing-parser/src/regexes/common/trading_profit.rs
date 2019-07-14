// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                          # sometimes there's whitespace before
        trading
        \s+
        profit
        \s+
        \(loss\)
        (,*\s+net)*
        (:)*
        \s*                          # sometimes there's whitespace after
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
        let match_examples = vec!["TRADING PROFIT (LOSS), NET:", "TRADING   PROFIT (LOSS):"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["profit", "net income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
