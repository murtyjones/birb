use regex::Regex;

pub mod cost_of_goods_sold;
pub mod depreciation_amortization;
pub mod earnings_per_share;
pub mod income_before_taxes;
pub mod income_tax_expense;
pub mod interest_income_or_expense;
pub mod legal_fees;
pub mod net_income;
pub mod operating_expenses;
pub mod shares_outstanding;
pub mod shares_used;

lazy_static! {
    pub static ref INCOME_STATEMENT_REGEXES: Vec<&'static Regex> = vec![
        &earnings_per_share::REGEX,
        &interest_income_or_expense::REGEX,
        &income_before_taxes::REGEX,
        &shares_outstanding::REGEX,
        &legal_fees::REGEX,
        &depreciation_amortization::REGEX,
        &net_income::REGEX,
        &income_tax_expense::REGEX,
        &cost_of_goods_sold::REGEX,
        &operating_expenses::REGEX,
    ];
}
