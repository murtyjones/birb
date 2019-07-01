// regex
use regex::{Regex, RegexBuilder};
use std::ascii::escape_default;

// test files
use crate::test_files::FILES;

lazy_static! {
    static ref INCOME_STATEMENT_HEADER_PATTERN: &'static str = r"
        ^
        (<b>)*                          # Optional closing tag
        (condensed)*\s*                 # 'condensed ' (optional)
        (consolidated)*\s*              # 'consolidated ' (optional)
        (condensed)*\s*                 # 'condensed ' (optional after consolidated)
        statement(s)*\s+                # 'statement' or 'statements' (with optional whitespace)
        of\s+                           # 'of '
        (income|operations|earnings)\s* # 'income' or 'operations' or 'earnings', possibly with a space
        (\(loss\))*                     # The word '(loss)' may be at the end
        (and\s+comprehensive\s+loss)*   # The term 'and comprehensive loss' may be at the end
        (and\s+comprehensive\s+income)* # The term 'and comprehensive income' may be at the end
        (</b>)*                         # Optional closing tag
        \s*                             # Optional whitespace
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
    use crate::test_files::MatchType;

    #[test]
    fn test_income_statement_known_header_regex_examples() {
        for i in 0..FILES.len() {
            let file = &FILES[i];
            if file.match_type == MatchType::Regex {
                assert!(INCOME_STATEMENT_HEADER_REGEX.is_match(&file.header_inner_html));
            }
        }
    }
}
