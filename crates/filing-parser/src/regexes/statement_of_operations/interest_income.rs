// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        interest\s+income(,\s+net)*
        \s*
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
            "Interest income",
            "interest income",
            "Interest income, net ",
            "Interest income, net",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec![
            "interest expense",
            "Note 2—Interest Income and Interest Expense",
        ];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
