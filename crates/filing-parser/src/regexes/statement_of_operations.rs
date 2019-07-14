use regex::Regex;

use crate::regexes::common::*;

/*
 * There should be at least this many regex matches that indicate
 * that a given table is an income statement. If less, return false.
 * This number should get larger over time as the regex patterns
 * become more accurate. If you find yourself lowering it...
 * Think about whether that is the right thing to do.
 */
pub static INCOME_STATEMENT_MIN_REQUIRED_REGEXES: i32 = 4;

lazy_static! {
    pub static ref INCOME_STATEMENT_REGEXES: Vec<&'static Regex> = vec![
        &compensation_and_benefits::REGEX,
        &earnings_per_share::REGEX,
        &interest_income_or_expense::REGEX,
        &income_before_taxes::REGEX,
        &shares_outstanding::REGEX,
        &professional_fees::REGEX,
        &legal_and_admin_fees::REGEX,
        &depreciation_amortization::REGEX,
        &net_income::REGEX,
        &income_tax_expense::REGEX,
        &total_expenses::REGEX,
        &cost_of_goods_sold::REGEX,
        &operating_expenses::REGEX,
        &selling_general_and_administrative::REGEX,
        &non_interest_income_expense::REGEX,
        &dividends_per_share::REGEX,
        &provision_for_bad_debts::REGEX,
        &operating_income::REGEX,
        &real_estate_taxes::REGEX,
        &revenues::REGEX,
        &research_and_development::REGEX,
        &management_fees::REGEX,
        &stock_based_compensation::REGEX,
    ];
}
