use regex::Regex;

pub mod earnings_per_share;
pub mod income_before_taxes;
pub mod interest_income;
pub mod shares_outstanding;
pub mod shares_used;

lazy_static! {
    pub static ref INCOME_STATEMENT_REGEXES: Vec<&'static Regex> = vec![
        &earnings_per_share::EARNINGS_PER_SHARE_REGEX,
        &interest_income::INTEREST_INCOME_REGEX,
        &income_before_taxes::OPER_INCOME_REGEX,
        &shares_outstanding::SHARES_OUTSTANDING_REGEX,
        &shares_used::SHARES_USED_REGEX,
    ];
}
