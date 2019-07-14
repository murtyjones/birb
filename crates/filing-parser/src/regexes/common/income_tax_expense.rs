// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                            # Sometimes there's whitespace before
        (federal\s+)*
        (\(benefit\)[\s/]+)*
        (
            provision\s+for\s+income\s+taxes
            |
            income\s+tax(\s+expense)*
            |
            income\s+tax\s+(\(expense\)\s+)*benefit
            |
            income\s+tax\s+expense\s+\(benefit\)
            |
            income\s+tax\s+benefit\s+\(expense\)
            |
            income\s+tax\s+\(provision\)\s+benefit
            |
            income\s+tax\s+provision(\s+|/)\(benefit\)
            |
            income\s+taxes
        )
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
            "(Benefit)/provision for income taxes",
            "Provision for income taxes",
            "Provision for income taxes  ",
            "Income tax expense",
            "Income tax (expense) benefit",
            "Income tax expense (benefit)",
            "Income tax benefit",
            "  Income tax",
            "Income tax (provision) benefit",
            "Income tax benefit (expense)",
            "Income taxes",
            "Income tax provision (benefit)",
            "Income tax provision/(benefit)",
            "Federal income tax expense (benefit)",
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
