// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref INTEREST_INCOME_PATTERN: &'static str = r"
        ^
        interest\s+income
        $
    ";
    pub static ref INTEREST_INCOME_REGEX: Regex = RegexBuilder::new(&INTEREST_INCOME_PATTERN)
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
    fn test_interest_income() {
        let match_examples = vec!["Interest income", "interest income"];
        for each in match_examples {
            assert!(INTEREST_INCOME_REGEX.is_match(each));
        }
        let no_match_examples = vec![
            "interest expense",
            "Note 2—Interest Income and Interest Expense",
        ];
        for each in no_match_examples {
            assert!(!INTEREST_INCOME_REGEX.is_match(each));
        }
    }
}
