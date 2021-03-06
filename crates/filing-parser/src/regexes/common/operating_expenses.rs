// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        (total\s+)*
        operating
        \s+
        expenses
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
        let match_examples = vec!["Operating expenses", "Total operating expenses"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["operating earnings", "operating"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
