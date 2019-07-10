// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        \s*                                           # sometimes there's whitespace before
        (basic\s+and\s+diluted\s+)*
        (net\s+)*
        (
            income(\s+\(loss\))*
            |
            earnings(\s+\(loss\))*
            |
            \(loss\)\s+(earnings|income)
            |
            loss
        )
        \s+
        per
        \s+
        (common\s+)*share
        (
             (\s+\(basic\))
             |
             (\s+[-–—]\s+(diluted|basic))
             |
             ((\s+[-–—])*\s+basic\s+and\s+diluted)*
             |
             ,\s+(basic|diluted)
        )*
        (:\s*)*
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
    fn test_earnings_per_share() {
        let match_examples = vec![
            "Earnings (loss) per share:",
            "Earnings per share (basic)",
            "Basic and diluted loss per share",
            "Net income per share – diluted",
            "Net income per share - basic",
            "Net income per share - basic:",
            "Earnings per common share:",
            "Loss per share basic and diluted: ",
            "Net (loss) earnings per share:",
            "Net Earnings Per Common Share, Basic",
            "Net Earnings Per Common Share, Diluted",
            "(Loss) income per common share - basic and diluted:",
            "   Income (loss) per share",
            "Earnings per share:",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["net earnings", "Note 12—Earnings Per Share"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
