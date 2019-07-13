// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                        # Sometimes there's whitespace before
        (consolidated\s+)*
        net
        \s+
        (
            (earnings|income|profit)(\s+\(loss\))*
            |
            loss
            |
            \(loss\)[\s/]+(earnings|income|profit)
        )
        (\s+attributable\s+to\s+(shareholders|unitholders))*
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
            "Net income",
            "Net income (loss)",
            "Net loss",
            "NET LOSS",
            "Net (loss) earnings",
            "Net Profit (Loss)",
            "Net (loss)/income",
            "  Net income",
            "Net income (loss) attributable to unitholders",
            "Consolidated Net Income",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share", "income"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
