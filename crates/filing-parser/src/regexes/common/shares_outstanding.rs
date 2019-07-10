// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (weighted[\s+|-])*
        average
        (\s+number\s+of)*
        \s+
        (
            (basic\s+)*common\s+
            |
            basic\s+and\s+diluted\s+
        )*
        (
            shares(\s+outstanding)*
        |
            outstanding\s+shares
        )

        (
            \s*[-–—]\s*basic\s+and\s+diluted
            |
            \s*[-–—]\s*(diluted|basic)
            |
            \s+\((basic|diluted)\)
            |
            ,\s+(basic|diluted)
        )*
        (\s+\(in\s+shares\))*
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
    fn test() {
        let match_examples = vec![
            "Weighted average number of shares outstanding",
            "Weighted average shares outstanding - basic",
            "Weighted average shares – basic and diluted",
            "Weighted average shares outstanding - diluted",
            "Weighted average common shares outstanding:",
            "Average shares outstanding (basic)",
            "Weighted average number of common shares outstanding - basic and diluted",
            "Weighted-average common shares outstanding",
            "Weighted average shares outstanding:",
            "Weighted-average number of basic common shares",
            "average shares",
            "Weighted average shares outstanding: ",
            "Weighted Average Common Shares Outstanding, Basic",
            "Weighted Average Common Shares Outstanding, Diluted",
            "Weighted average shares - basic (in shares)",
            "Average shares outstanding—basic (in shares)",
            "Weighted average outstanding shares:",
            "Weighted average basic and diluted shares outstanding",
        ];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share", "shares held", "shares"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
