use regex::Regex;

use crate::regexes::common::*;

lazy_static! {
    pub static ref INCOME_STATEMENT_REGEXES: Vec<&'static Regex> = vec![
        &earnings_per_share::REGEX,
        &interest_income_or_expense::REGEX,
        &income_before_taxes::REGEX,
        &shares_outstanding::REGEX,
        &legal_and_admin_fees::REGEX,
        &depreciation_amortization::REGEX,
        &net_income::REGEX,
        &income_tax_expense::REGEX,
        &cost_of_goods_sold::REGEX,
        &operating_expenses::REGEX,
        &selling_general_and_administrative::REGEX,
        &non_interest_income_expense::REGEX,
        &dividends_per_share::REGEX,
        &provision_for_bad_debts::REGEX,
        &income_from_continuing_operations::REGEX,
        &real_estate_taxes::REGEX,
        &revenues::REGEX,
    ];
}
