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
        (
            \s+for\s+the\s+
            (three|six|nine)
            \s+
            months\s+ended\s+
            (March|June|September|December)+
            \s+
            (31|30)
            ,*
        )*
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
            "STATEMENTS\nOF INCOME", // edgar/data/101295/0001171843-17-002984.txt
            "unaudited condensed statements of operations",
            "CONSOLIDATED STATEMENTS OF COMPREHENSIVE LOSS", // this was needed for one specific filing, but i can't remember which and im not sure this is a pattern we want to match against given that this doesn't always relate to and income statement table :/
            "CONSOLIDATED STATEMENTS OF OPERATIONS (UNAUDITED)",
            "Condensed Consolidated Statements of\nOperations for the Three Months Ended March 31,", // edgar/data/1009891/0001193805-17-000932.txt
        ];
        for each in examples {
            assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(each));
        }
    }
}
