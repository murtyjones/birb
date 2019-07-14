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
        &cost_of_goods_sold::REGEX,
        &data_processing::REGEX,
        &depreciation_amortization::REGEX,
        &dividends_per_share::REGEX,
        &earnings_per_share::REGEX,
        &income_before_taxes::REGEX,
        &income_tax_expense::REGEX,
        &interest_income_or_expense::REGEX,
        &legal_and_admin_fees::REGEX,
        &management_fees::REGEX,
        &net_income_controlling_interests::REGEX,
        &net_income::REGEX,
        &non_interest_income_expense::REGEX,
        &operating_expenses::REGEX,
        &operating_income::REGEX,
        &professional_fees::REGEX,
        &provision_for_bad_debts::REGEX,
        &real_estate_taxes::REGEX,
        &research_and_development::REGEX,
        &revenues::REGEX,
        &selling_general_and_administrative::REGEX,
        &shares_outstanding::REGEX,
        &shares_used::REGEX,
        &stock_based_compensation::REGEX,
        &total_expenses::REGEX,
        &trading_profit::REGEX,
    ];
}
