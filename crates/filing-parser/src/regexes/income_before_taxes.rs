// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref OPER_INCOME_PATTERN: &'static str = r"
        ^
        (operating\s+)*
        income
        \s+
        before
        \s+
        (
            income\s+taxes
            |
            income\s+tax\s+expense
        )
        $
    ";
    pub static ref OPER_INCOME_REGEX: Regex = RegexBuilder::new(&OPER_INCOME_PATTERN)
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
    fn test_shares_outstanding() {
        let match_examples = vec![
            "Operating income before income taxes",
            "Income before income tax expense",
        ];
        for each in match_examples {
            assert!(OPER_INCOME_REGEX.is_match(each));
        }
        let no_match_examples = vec![
            "Net Income",
            "income",
            "Income before income tax expense / operating margin",
        ];
        for each in no_match_examples {
            assert!(!OPER_INCOME_REGEX.is_match(each));
        }
    }
}
