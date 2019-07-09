use regex::Regex;

pub mod depreciation_amortization;
pub mod earnings_per_share;
pub mod income_before_taxes;
pub mod interest_income;
pub mod legal_fees;
pub mod shares_outstanding;
pub mod shares_used;

lazy_static! {
    pub static ref INCOME_STATEMENT_REGEXES: Vec<&'static Regex> = vec![
        &earnings_per_share::REGEX,
        &interest_income::REGEX,
        &income_before_taxes::REGEX,
        &shares_outstanding::REGEX,
        &legal_fees::REGEX,
//        &depreciation_amortization::REGEX,
    ];
}
