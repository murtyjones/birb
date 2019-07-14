// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*               # sometimes there's whitespace before
        (Selling,\s+)*
        (
            general(,)*\s+(and|&)\s+administrative(\s+expenses)*
            |
            G\s+&\s+A
        )
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
            "  Selling, general and administrative",
            "General and administrative expenses",
            "General and administrative ",
            "Selling, general and administrative expenses",
            "General and administrative",
            "Selling, G & A",
            "   General & administrative",
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
