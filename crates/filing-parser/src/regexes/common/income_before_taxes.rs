// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (operating\s+)*
        (
            (income|profit)(\s+\(loss\))*
            |
            \(loss\)[\s/]+(earnings|income)    # eg. `(loss) earnings` or `(loss)/earnings`
            |
            loss
        )
        \s+
        before\s+
        (\(benefit\)/provision\s+for\s+)*
        (
            income\s+taxes
            |
            income\s+tax\s+expense
            |
            taxes\s+on\s+income
            |
            income\s+tax\s+\(expense\)\s+benefit
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
            "Operating income before income taxes",
            "Income before income tax expense",
            "Income (loss) before income taxes",
            "(Loss) earnings before income taxes",
            "Loss Before Income Taxes",
            "Profit (Loss) before Income Taxes",
            "Income (loss) before taxes on income",
            "(Loss)/income before (benefit)/provision for income taxes",
            "Income (loss) before income tax (expense) benefit",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec![
            "Net Income",
            "income",
            "Income before income tax expense / operating margin",
        ];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
