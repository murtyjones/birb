// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                          # sometimes there's whitespace before
        (net\s+)*
        (interest|financial|investment)
        \s+
        (
            (income|expense(s)*)
            |
            expense\s\(income\)
            |
            income\s\((expense|loss)\)
            |
            \(expense\)
        )
        (,\s+net)*
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
        let match_examples = vec![
            "Net Interest Income",
            "Interest income",
            "interest income",
            "Interest income, net ",
            "Interest income, net",
            "Interest expense (income), net",
            "Interest expense",
            "Interest expense, net",
            "  Interest expense",
            "Interest income (expense), net",
            "Financial income",
            "Financial expenses",
            "Interest Income:",
            "Investment Income:",
            "  Interest (expense)",
            "INVESTMENT INCOME (EXPENSE):",
            "INVESTMENT   INCOME (LOSS)",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["Note 2—Interest Income and Interest Expense"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
