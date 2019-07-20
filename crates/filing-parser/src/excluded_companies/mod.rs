// regex
use regex::Regex;

pub mod income_statement;
pub mod sec_header;

pub struct ExcludedCompany {
    pub cik: &'static str,
    pub name: &'static str,
    pub excludable_name: Regex,
}

pub fn build_excl_name(pat: &'static str) -> Regex {
    let insensitive = format!("(?i){}", pat);
    Regex::new(&insensitive).expect("Couldn't build income statement regex!")
}
