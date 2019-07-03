// regex
use regex::{Regex, RegexBuilder};

lazy_static! {
    static ref INCOME_STATEMENT_HEADER_PATTERN: &'static str = r"
        ^
        (<b>)*                                # Optional closing tag
        (unaudited)*\s*                       # 'unaudited ' (optional)
        (condensed)*\s*                       # 'condensed ' (optional)
        (consolidated)*\s*                    # 'consolidated ' (optional)
        (condensed)*\s*                       # 'condensed ' (optional after consolidated)
        statement(s)*\s+                      # 'statement' or 'statements' (with optional whitespace)
        of\s+                                 # 'of '
        (comprehensive)*\s*                   # 'comprehensive ' (optional)
        (income|operations|earnings|loss)\s*  # 'income' or 'operations' or 'earnings' or 'loss', possibly with a space
        (\(loss\))*                           # The word '(loss)' may be at the end
        (and\s+comprehensive\s+loss)*         # The term 'and comprehensive loss' may be at the end
        (and\s+comprehensive\s+income)*       # The term 'and comprehensive income' may be at the end
        (</b>)*                               # Optional closing tag
        \s*                                   # Optional whitespace
        (\(unaudited\)\s*)*                   # Optional '(unaudited)'
        $
    ";
    pub static ref INCOME_STATEMENT_HEADER_REGEX: Regex =
        RegexBuilder::new(&INCOME_STATEMENT_HEADER_PATTERN)
            .case_insensitive(true)
            .multi_line(true)
            .ignore_whitespace(true)
            .build()
            .expect("Couldn't build income statement regex!");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_files::get_files;
    use crate::test_files::MatchType;

    #[test]
    fn test_income_statement_known_header_regex_examples() {
        let files = get_files();
        for file in files {
            if file.match_type == MatchType::Regex {
                assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&file.header_inner_html.unwrap()));
            }
        }
    }

    /// test examples for which we dont want to process the full doc
    #[test]
    fn test_further_examples() {
        let examples = vec![
            "unaudited condensed statements of operations",
            "CONLIDATED STATEMENTS OF COMPREHENSIVE LOSS",
            "CONSOLIDATED STATEMENTS OF OPERATIONS (UNAUDITED)",
        ];
        for each in examples {
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(each));
        }
    }
}
