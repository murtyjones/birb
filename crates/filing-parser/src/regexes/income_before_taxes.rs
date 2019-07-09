// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (operating\s+)*
        (
            income(\s+\(loss\))*
            |
            \(loss\)\s+earnings
            |
            loss
        )
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
