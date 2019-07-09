// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref PATTERN: &'static str = r"
        ^
        Less:\s+Allocation\s+of\s+earnings\s+\(loss\)\s+to\s+participating\s+securities
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
        let match_examples =
            vec!["Less: Allocation of earnings (loss) to participating securities"];
        for each in match_examples {
            assert!(REGEX.is_match(each));
        }
        let no_match_examples = vec!["earnings per share"];
        for each in no_match_examples {
            assert!(!REGEX.is_match(each));
        }
    }
}
