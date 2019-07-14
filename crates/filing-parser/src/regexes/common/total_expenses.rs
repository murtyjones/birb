// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                            # Sometimes there's whitespace before
        total
        (\s+costs\s+and)*
        \s+
        expense(s)*
        (\s+after\s+reimbursement)*
        \s*                                            # Sometimes there's whitespace after
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
            "total expenses",
            "total expense",
            "Total expenses   after reimbursement",
            "Total costs and expenses",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share", "income", "net income", "tax"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
