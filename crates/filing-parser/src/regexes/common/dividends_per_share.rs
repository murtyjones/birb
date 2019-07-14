// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                          # sometimes there's whitespace before
        (cash\s+)*
        (dividends|distributions)
        (\s+(declared|paid))*
        \s+
        per
        \s+
        (
            share
            |
            common\s+share
            |
            share\s+of\s+common\s+stock
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
        let match_examples = vec![
            "Dividends declared per common share",
            "Dividends declared per share",
            "Distributions declared per share",
            "Dividends per share",
            "Cash dividends paid per share of common stock",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
